use crate::{
    game_mode_state::GameModeState, game_type::GameType, input::Input, menu_state::MenuState,
    piece::Piece, play_state::PlayState, random::Random,
};
use bitvec::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub struct GameplayState {
    pub nmi_on: bool,
    pub dead: bool,
    pub previous_input: Input,
    pub score: u32,
    pub random: Random,
    pub current_piece_x: i8,
    pub current_piece_y: i8,
    pub fall_timer: u8,
    pub game_mode_state: GameModeState,
    pub fall_autorepeat: i8,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub rendered_row_counter: u8,
    pub line_count: u16,
    pub play_state: PlayState,
    pub shift_autorepeat: u8,
    pub tiles: BitArr!(for 0x100),
    pub level: u8,
    pub hold_down_points: u8,
    pub line_index: u8,
    pub cleared_lines: u8,
    pub game_type: GameType,
    pub update_lines_delay: u8,
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
            nmi_on: menu_state.nmi_on,
            dead: false,
            previous_input: menu_state.previous_input,
            score: 0,
            random: menu_state.random.clone(),
            current_piece_x: 5,
            current_piece_y: 0,
            fall_timer: 0,
            game_mode_state: GameModeState::HandleGameplay,
            fall_autorepeat: -96,
            current_piece: menu_state.current_piece,
            next_piece: menu_state.next_piece,
            rendered_row_counter: 0,
            line_count: match menu_state.game_type {
                GameType::A => 0,
                GameType::B => 25,
            },
            play_state: PlayState::MoveTetrimino,
            shift_autorepeat: 0,
            tiles: menu_state.tiles,
            level: menu_state.selected_level,
            hold_down_points: 0,
            line_index: 0,
            cleared_lines: 0,
            game_type: menu_state.game_type,
            update_lines_delay: 0,
            frame_counter: menu_state.frame_counter,
            paused: false,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> bool {
        self.tiles[y * 10 + x]
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: bool) {
        self.tiles.set(y * 10 + x, tile);
    }

    pub fn step(&mut self, input: Input) {
        if self.dead {
            return;
        }

        if self.nmi_on {
            if !self.paused {
                if self.play_state == PlayState::DoNothing {
                    if self.frame_counter == 0 {
                        self.update_lines_delay -= 1;
                        if self.update_lines_delay == 0 {
                            self.play_state = PlayState::UpdateLinesAndStatistics;
                        }
                    }
                    self.rendered_row_counter = 0;
                } else {
                    if self.rendered_row_counter < 20 {
                        self.rendered_row_counter += 4;
                    }
                }
            }

            self.frame_counter = (self.frame_counter + 1) % 4;
            self.random.step();
        }

        if self.paused {
            if input.difference(self.previous_input) == Input::Start {
                self.rendered_row_counter = 0;
                self.game_mode_state = GameModeState::Unpause;
                self.paused = false;
            }
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
                if input.difference(self.previous_input).get(Input::Start) {
                    self.paused = true;
                }
                self.game_mode_state = GameModeState::HandleGameplay;
                true
            }
            GameModeState::Unpause => {
                self.game_mode_state = GameModeState::HandleGameplay;
                true
            }
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

        if self.rendered_row_counter >= 20 {
            for (tile_offset_x, tile_offset_y) in self.current_piece.get_tile_offsets() {
                let x = i16::from(self.current_piece_x) + i16::from(*tile_offset_x);
                let y = i16::from(self.current_piece_y) + i16::from(*tile_offset_y);
                let offset = (y * 10 + x) as u8;
                self.set_tile((offset % 10).into(), (offset / 10).into(), true);
            }

            self.line_index = 0;
            self.update_playfield();
            self.play_state = PlayState::CheckForCompletedRows;
        }
    }

    fn check_for_completed_rows(&mut self) {
        if self.rendered_row_counter < 20 {
            return;
        }

        let general_counter2 = if self.current_piece_y < 2 {
            0
        } else {
            self.current_piece_y as u8 - 2
        } + self.line_index;
        let general_counter = general_counter2 * 10;

        for x in 0..10 {
            if !self.get_tile(x, general_counter2.into()) {
                self.increment_line_index();
                return;
            }
        }

        self.cleared_lines += 1;

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
            self.rendered_row_counter = 0;
            self.update_lines_delay = 5;
            self.play_state = PlayState::DoNothing;
            if self.cleared_lines == 0 {
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
                for _ in 0..self.cleared_lines {
                    self.line_count += 1;
                    self.line_count %= 10000;
                    if self.line_count % 10 == 0 {
                        let lines_middle_digits = (self.line_count / 10) as u8 % 100;
                        let target_level = lines_middle_digits + 6 * (lines_middle_digits / 10);
                        if self.level < target_level {
                            self.level += 1;
                        }
                    }
                }
            }
            GameType::B => {
                self.line_count = if self.line_count > u16::from(self.cleared_lines) {
                    self.line_count - u16::from(self.cleared_lines)
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

        if self.cleared_lines != 0 {
            self.score += u32::from(Self::POINTS_TABLE[self.cleared_lines as usize])
                * u32::from(self.level + 1);
            if self.score > 999999 {
                self.score = 999999;
            }
        }

        self.cleared_lines = 0;
        self.play_state = PlayState::SkipTo7;
    }

    fn spawn_next_tetrimino(&mut self) {
        if self.rendered_row_counter < 20 {
            return;
        }
        self.fall_timer = 0;
        self.current_piece_y = 0;
        self.play_state = PlayState::MoveTetrimino;
        self.current_piece_x = 5;
        self.current_piece = self.next_piece;
        self.next_piece = self.random.next_piece();
        self.fall_autorepeat = 0;
    }

    fn shift_tetrimino(&mut self, input: Input) {
        let original_y = self.current_piece_x;
        if input.get(Input::Down) {
            return;
        }

        let pressed_input = input.difference(self.previous_input);
        if pressed_input & (Input::Left | Input::Right) == 0 {
            if input & (Input::Left | Input::Right) == 0 {
                return;
            }

            self.shift_autorepeat += 1;
            if self.shift_autorepeat < 16 {
                return;
            }
            self.shift_autorepeat = 10;
        } else {
            self.shift_autorepeat = 0;
        }

        if input.get(Input::Right) {
            self.current_piece_x += 1;
            if !self.is_position_valid() {
                self.current_piece_x = original_y;
                self.shift_autorepeat = 16;
            }
        } else if input.get(Input::Left) {
            self.current_piece_x -= 1;
            if !self.is_position_valid() {
                self.current_piece_x = original_y;
                self.shift_autorepeat = 16;
            }
        }
    }

    fn is_position_valid(&mut self) -> bool {
        for (tile_offset_x, tile_offset_y) in self.current_piece.get_tile_offsets() {
            let x = tile_offset_x + self.current_piece_x;
            let y = tile_offset_y + self.current_piece_y;
            if x < 0 || x >= 10 || y >= 20 {
                return false;
            }

            let y = i16::from(*tile_offset_x)
                + i16::from(*tile_offset_y * 10)
                + i16::from(self.current_piece_y as u8) * 10
                + i16::from(self.current_piece_x as u8);
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
        if self.fall_autorepeat < 0 {
            if !pressed_input.get(Input::Down) {
                self.fall_autorepeat += 1;
                return;
            }
            self.fall_autorepeat = 0;
        }

        if self.fall_autorepeat == 0 && !input.get(Input::Left) && !input.get(Input::Right) {
            if pressed_input.get(Input::Down)
                && !pressed_input.get(Input::Left)
                && !pressed_input.get(Input::Right)
                && !pressed_input.get(Input::Up)
            {
                self.fall_autorepeat = 1;
            }
        } else if !(input.get(Input::Down)
            && !input.get(Input::Left)
            && !input.get(Input::Right)
            && !input.get(Input::Up))
        {
            self.fall_autorepeat = 0;
            self.hold_down_points = 0;
        } else if self.fall_autorepeat != 0 {
            self.fall_autorepeat += 1;
            if self.fall_autorepeat >= 3 {
                self.fall_autorepeat = 1;
                self.hold_down_points += 1;

                self.fall_timer = 0;
                let original_y = self.current_piece_y;
                self.current_piece_y += 1;
                if !self.is_position_valid() {
                    self.current_piece_y = original_y;
                    self.play_state = PlayState::LockTetrimino;
                    self.update_playfield();
                }
                return;
            }
        }

        let frames_per_drop = if self.level < 0x1d {
            Self::FRAMES_PER_DROP_TABLE[self.level as usize]
        } else {
            1
        };
        if self.fall_timer >= frames_per_drop {
            self.fall_timer = 0;
            let original_y = self.current_piece_y;
            self.current_piece_y += 1;
            if !self.is_position_valid() {
                self.current_piece_y = original_y;
                self.play_state = PlayState::LockTetrimino;
                self.update_playfield();
            }
        }
    }

    fn update_playfield(&mut self) {
        let highest_row_to_update = if self.current_piece_y >= 2 {
            self.current_piece_y as u8 - 2
        } else {
            0
        };
        self.rendered_row_counter = u8::min(self.rendered_row_counter, highest_row_to_update);
    }
}
