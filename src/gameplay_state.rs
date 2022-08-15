use crate::{
    game_mode_state::GameModeState, game_type::GameType, input::Input, menu_state::MenuState,
    piece::Piece, play_state::PlayState, random::Random,
};
use bitvec::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub struct GameplayState {
    pub do_nmi: bool,
    pub dead: bool,
    pub previous_input: Input,
    pub score: u32,
    pub random: Random,
    pub tetrimino_x: i8,
    pub tetrimino_y: i8,
    pub fall_timer: u8,
    pub game_mode_state: GameModeState,
    pub autorepeat_y: i8,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub vram_row: u8,
    pub lines: u16,
    pub play_state: PlayState,
    pub autorepeat_x: u8,
    pub playfield: BitArr!(for 0x100),
    pub level_number: u8,
    pub hold_down_points: u8,
    pub line_index: u8,
    pub completed_lines: u8,
    pub game_type: GameType,
    pub row_y: u8,
    pub frame_counter: u8,
    pub paused: bool,
}

impl GameplayState {
    const FRAMES_PER_DROP_TABLE: [u8; 30] = [
        48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 1,
    ];
    const POINTS_TABLE: [u16; 5] = [0, 40, 100, 300, 1200];

    pub fn from_menu_state(menu_state: &MenuState) -> Self {
        Self {
            do_nmi: menu_state.do_nmi,
            dead: false,
            previous_input: menu_state.previous_input,
            score: 0,
            random: menu_state.random.clone(),
            tetrimino_x: 5,
            tetrimino_y: 0,
            fall_timer: 0,
            game_mode_state: GameModeState::HandleGameplay,
            autorepeat_y: -96,
            current_piece: menu_state.current_piece,
            next_piece: menu_state.next_piece,
            vram_row: 0,
            lines: match menu_state.game_type {
                GameType::A => 0,
                GameType::B => 25,
            },
            play_state: PlayState::MoveTetrimino,
            autorepeat_x: 0,
            playfield: menu_state.playfield,
            level_number: menu_state.start_level,
            hold_down_points: 0,
            line_index: 0,
            completed_lines: 0,
            game_type: menu_state.game_type,
            row_y: 0,
            frame_counter: menu_state.frame_counter,
            paused: false,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> bool {
        self.playfield[y * 10 + x]
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: bool) {
        self.playfield.set(y * 10 + x, tile);
    }

    pub fn step(&mut self, input: Input) {
        if self.dead {
            return;
        }

        if self.do_nmi {
            self.render();
            self.frame_counter = (self.frame_counter + 1) % 4;
            self.random.step();
        }

        if self.paused {
            self.pause_loop(input);
            if self.paused {
                self.previous_input = input.clone();
                return;
            }
        }

        loop {
            let force_end_loop = self.play_and_ending_high_score(input);
            if force_end_loop || self.dead || self.paused {
                self.previous_input = input.clone();
                return;
            }
        }
    }

    fn play_and_ending_high_score(&mut self, input: Input) -> bool {
        match self.game_mode_state {
            GameModeState::HandleGameplay => {
                self.fall_timer += 1;
                self.branch_on_play_state_player1(input);
                self.game_mode_state = GameModeState::HandleStartButton;
                input == Input::Right | Input::Left | Input::Down
            }
            GameModeState::HandleStartButton => {
                self.start_button_handling(input);
                self.game_mode_state = GameModeState::HandleGameplay;
                true
            }
            GameModeState::Unpause => {
                self.game_mode_state = GameModeState::HandleGameplay;
                true
            }
        }
    }

    fn start_button_handling(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);

        if pressed_input.get(Input::Start) {
            self.paused = true;
        }
    }

    fn pause_loop(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Start {
            self.vram_row = 0;
            self.game_mode_state = GameModeState::Unpause;
            self.paused = false;
        }
    }

    fn branch_on_play_state_player1(&mut self, input: Input) {
        match self.play_state {
            PlayState::MoveTetrimino => {
                self.shift_tetrimino(input);
                self.rotate_tetrimino(input);
                self.drop_tetrimino(input);
            }
            PlayState::LockTetrimino => self.lock_tetrimino(),
            PlayState::CheckForCompletedRows => self.check_for_completed_rows(),
            PlayState::DoNothing => (),
            PlayState::UpdateLinesAndStatistics => self.update_lines_and_statistics(),
            PlayState::SkipTo7 => {
                self.play_state = PlayState::SkipTo8;
            }
            PlayState::SkipTo8 => {
                self.play_state = PlayState::SpawnNextTetrimino;
            }
            PlayState::SpawnNextTetrimino => self.spawn_next_tetrimino(),
        }
    }

    fn lock_tetrimino(&mut self) {
        if !self.is_position_valid() {
            self.dead = true;
            return;
        }

        if self.vram_row >= 32 {
            for (tile_offset_x, tile_offset_y) in self.current_piece.get_tile_offsets() {
                let x = i16::from(self.tetrimino_x) + i16::from(*tile_offset_x);
                let y = i16::from(self.tetrimino_y) + i16::from(*tile_offset_y);
                let offset = (y * 10 + x) as u8;
                self.set_tile((offset % 10).into(), (offset / 10).into(), true);
            }

            self.line_index = 0;
            self.update_playfield();
            self.play_state = PlayState::CheckForCompletedRows;
        }
    }

    fn check_for_completed_rows(&mut self) {
        if self.vram_row < 32 {
            return;
        }

        let general_counter2 = if self.tetrimino_y < 2 {
            0
        } else {
            self.tetrimino_y as u8 - 2
        } + self.line_index;
        let general_counter = general_counter2 * 10;

        for x in 0..10 {
            if !self.get_tile(x, general_counter2.into()) {
                self.increment_line_index();
                return;
            }
        }

        self.completed_lines += 1;

        let mut y = u8::wrapping_sub(general_counter, 1);
        if y == 255 {
            y = 245;
        }
        loop {
            self.set_tile(
                (y % 10).into(),
                (y / 10 + 1).into(),
                self.get_tile((y % 10).into(), (y / 10).into()),
            );
            if y == 0 {
                break;
            }
            y = u8::wrapping_sub(y, 1);
        }

        for x in 0..10 {
            self.set_tile(x, 0, false);
        }

        self.current_piece = Piece::None;
        self.increment_line_index();
    }

    fn increment_line_index(&mut self) {
        self.line_index += 1;
        if self.line_index == 4 {
            self.vram_row = 0;
            self.row_y = 0;
            self.play_state = PlayState::DoNothing;
            if self.completed_lines == 0 {
                self.play_state = PlayState::UpdateLinesAndStatistics;
            }
        }
    }

    fn update_lines_and_statistics(&mut self) {
        fn to_bcd(number: u8) -> u8 {
            number + 6 * (number / 10)
        }

        fn from_bcd(number: u16) -> u16 {
            let mut result = 0;
            let mut shifted_number = number;
            let mut multiplier = 1;
            while shifted_number > 0 {
                result += (shifted_number % 16) * multiplier;
                shifted_number >>= 4;
                multiplier *= 10;
            }
            result
        }

        match self.game_type {
            GameType::A => {
                for _ in 0..self.completed_lines {
                    self.lines += 1;
                    self.lines %= 10000;
                    if self.lines % 10 == 0 {
                        let lines_middle_digits = (self.lines / 10) as u8 % 100;
                        let target_level = lines_middle_digits + 6 * (lines_middle_digits / 10);
                        if self.level_number < target_level {
                            self.level_number += 1;
                        }
                    }
                }
            }
            GameType::B => {
                self.lines = if self.lines > u16::from(self.completed_lines) {
                    self.lines - u16::from(self.completed_lines)
                } else {
                    0
                };
            }
        }

        if self.hold_down_points >= 2 {
            // buggy score addition logic from base game
            let low_digits = from_bcd(
                u16::from(to_bcd((self.score % 100) as u8)) + u16::from(self.hold_down_points - 1),
            );
            self.score -= self.score % 100;
            self.score += u32::from(low_digits);
            if low_digits >= 100 {
                self.score -= self.score % 10;
            }
        }
        self.hold_down_points = 0;

        if self.completed_lines != 0 {
            self.score += u32::from(Self::POINTS_TABLE[self.completed_lines as usize])
                * u32::from(self.level_number + 1);
            if self.score > 999999 {
                self.score = 999999;
            }
        }

        self.completed_lines = 0;
        self.play_state = PlayState::SkipTo7;
    }

    fn spawn_next_tetrimino(&mut self) {
        if self.vram_row < 32 {
            return;
        }
        self.fall_timer = 0;
        self.tetrimino_y = 0;
        self.play_state = PlayState::MoveTetrimino;
        self.tetrimino_x = 5;
        self.current_piece = self.next_piece;
        self.next_piece = self.random.next_piece();
        self.autorepeat_y = 0;
    }

    fn shift_tetrimino(&mut self, input: Input) {
        let original_y = self.tetrimino_x;
        if input.get(Input::Down) {
            return;
        }

        let pressed_input = input.difference(self.previous_input);
        if pressed_input & (Input::Left | Input::Right) == 0 {
            if input & (Input::Left | Input::Right) == 0 {
                return;
            }

            self.autorepeat_x += 1;
            if self.autorepeat_x < 16 {
                return;
            }
            self.autorepeat_x = 10;
        } else {
            self.autorepeat_x = 0;
        }

        if input.get(Input::Right) {
            self.tetrimino_x += 1;
            if !self.is_position_valid() {
                self.tetrimino_x = original_y;
                self.autorepeat_x = 16;
            }
        } else if input.get(Input::Left) {
            self.tetrimino_x -= 1;
            if !self.is_position_valid() {
                self.tetrimino_x = original_y;
                self.autorepeat_x = 16;
            }
        }
    }

    fn is_position_valid(&mut self) -> bool {
        for (tile_offset_x, tile_offset_y) in self.current_piece.get_tile_offsets() {
            let x = tile_offset_x + self.tetrimino_x;
            let y = tile_offset_y + self.tetrimino_y;
            if x < 0 || x >= 10 || y >= 20 {
                return false;
            }

            let y = i16::from(*tile_offset_x)
                + i16::from(*tile_offset_y * 10)
                + i16::from(self.tetrimino_y as u8) * 10
                + i16::from(self.tetrimino_x as u8);
            if self.get_tile((y as u8 % 10).into(), (y as u8 / 10).into()) {
                return false;
            }
        }

        true
    }

    fn rotate_tetrimino(&mut self, input: Input) {
        let original_y = self.current_piece;
        let pressed_input = input.difference(self.previous_input);
        if pressed_input.get(Input::A) {
            self.current_piece = self.current_piece.get_clockwise_rotation();
            if !self.is_position_valid() {
                self.current_piece = original_y;
            }
        } else if pressed_input.get(Input::B) {
            self.current_piece = self.current_piece.get_counterclockwise_rotation();
            if !self.is_position_valid() {
                self.current_piece = original_y;
            }
        }
    }

    fn drop_tetrimino(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if self.autorepeat_y < 0 {
            if !pressed_input.get(Input::Down) {
                self.autorepeat_y += 1;
                return;
            }
            self.autorepeat_y = 0;
        }
        if self.autorepeat_y == 0 {
            if input.get(Input::Left) || input.get(Input::Right) {
                self.lookup_drop_speed();
                return;
            }
            if pressed_input.get(Input::Down)
                && !pressed_input.get(Input::Left)
                && !pressed_input.get(Input::Right)
                && !pressed_input.get(Input::Up)
            {
                self.autorepeat_y = 1;
            }
            self.lookup_drop_speed();
            return;
        }

        if !(input.get(Input::Down)
            && !input.get(Input::Left)
            && !input.get(Input::Right)
            && !input.get(Input::Up))
        {
            self.autorepeat_y = 0;
            self.hold_down_points = 0;
            self.lookup_drop_speed();
            return;
        }
        self.autorepeat_y += 1;
        if self.autorepeat_y >= 3 {
            self.autorepeat_y = 1;
            self.hold_down_points += 1;

            self.fall_timer = 0;
            let original_y = self.tetrimino_y;
            self.tetrimino_y += 1;
            if !self.is_position_valid() {
                self.tetrimino_y = original_y;
                self.play_state = PlayState::LockTetrimino;
                self.update_playfield();
            }
        } else {
            self.lookup_drop_speed();
            return;
        }
    }

    fn lookup_drop_speed(&mut self) {
        let frames_per_drop = if self.level_number < 0x1d {
            Self::FRAMES_PER_DROP_TABLE[self.level_number as usize]
        } else {
            1
        };
        if self.fall_timer >= frames_per_drop {
            self.fall_timer = 0;
            let original_y = self.tetrimino_y;
            self.tetrimino_y += 1;
            if !self.is_position_valid() {
                self.tetrimino_y = original_y;
                self.play_state = PlayState::LockTetrimino;
                self.update_playfield();
            }
        }
    }

    fn update_playfield(&mut self) {
        let highest_row_to_update = if self.tetrimino_y >= 2 {
            self.tetrimino_y as u8 - 2
        } else {
            0
        };
        self.vram_row = u8::min(self.vram_row, highest_row_to_update);
    }

    fn render(&mut self) {
        if self.paused {
            return;
        }

        if self.play_state == PlayState::DoNothing {
            if self.frame_counter == 0 {
                self.row_y += 1;
                if self.row_y >= 5 {
                    self.play_state = PlayState::UpdateLinesAndStatistics;
                }
            }
            self.vram_row = 0;
        } else {
            if self.vram_row < 21 {
                self.vram_row += 4;
                if self.vram_row >= 20 {
                    self.vram_row = 32;
                }
            }
        }
    }
}
