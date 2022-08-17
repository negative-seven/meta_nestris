use crate::{
    game_mode_state::GameModeState, game_type::GameType, input::Input, piece::Piece,
    play_state::PlayState, random::Random,
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
    pub checked_row_offset: u8,
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

    pub fn new(
        random: &Random,
        frame_counter: u8,
        previous_input: Input,
        game_type: GameType,
        level: u8,
        tiles: &BitArr!(for 0x100),
        first_piece: Piece,
        second_piece: Piece,
    ) -> Self {
        Self {
            nmi_on: true,
            dead: false,
            previous_input: previous_input,
            score: 0,
            random: random.clone(),
            current_piece_x: 5,
            current_piece_y: 0,
            fall_timer: 0,
            game_mode_state: GameModeState::HandleGameplay,
            fall_autorepeat: -96,
            current_piece: first_piece,
            next_piece: second_piece,
            rendered_row_counter: 0,
            line_count: match game_type {
                GameType::A => 0,
                GameType::B => 25,
            },
            play_state: PlayState::MoveTetrimino,
            shift_autorepeat: 0,
            tiles: tiles.clone(),
            level,
            hold_down_points: 0,
            checked_row_offset: 0,
            cleared_lines: 0,
            game_type: game_type,
            update_lines_delay: 0,
            frame_counter: frame_counter,
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
            self.play_and_ending_high_score(input);
            self.previous_input = input.clone();
            return;
        }
    }

    fn play_and_ending_high_score(&mut self, input: Input) {
        if self.game_mode_state == GameModeState::HandleGameplay {
            self.fall_timer += 1;
            self.branch_on_play_state_player1(input);
            self.game_mode_state = GameModeState::HandleStartButton;

            if self.dead || input == Input::Right | Input::Left | Input::Down
            // bug from base game - holding right, left and down causes the frame to end early
            {
                return;
            }
        }

        if self.game_mode_state == GameModeState::HandleStartButton {
            if input.difference(self.previous_input).get(Input::Start) {
                self.paused = true;
            }
        }

        self.game_mode_state = GameModeState::HandleGameplay;
    }

    fn branch_on_play_state_player1(&mut self, input: Input) {
        match self.play_state {
            PlayState::MoveTetrimino => {
                self.shift_tetrimino(input);
                self.rotate_tetrimino(input);
                self.drop_tetrimino(input);
            }
            PlayState::LockTetrimino => self.lock_tetrimino(),
            PlayState::CheckForCompletedRows => self.check_completed_row(),
            PlayState::DoNothing => (),
            PlayState::UpdateLinesAndStatistics => self.update_lines_and_statistics(),
            PlayState::Wait2UntilSpawnNextTetrimino => {
                self.play_state = PlayState::Wait1UntilSpawnNextTetrimino;
            }
            PlayState::Wait1UntilSpawnNextTetrimino => {
                self.play_state = PlayState::SpawnNextTetrimino;
            }
            PlayState::SpawnNextTetrimino => self.spawn_next_tetrimino(),
        }
    }

    fn lock_tetrimino(&mut self) {
        if !self.try_set_piece_and_position(
            self.current_piece,
            self.current_piece_x,
            self.current_piece_y,
        ) {
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

            self.checked_row_offset = 0;
            self.update_playfield();
            self.play_state = PlayState::CheckForCompletedRows;
        }
    }

    fn check_completed_row(&mut self) {
        if self.rendered_row_counter < 20 {
            return;
        }

        let checked_row = if self.current_piece_y < 2 {
            0
        } else {
            self.current_piece_y as u8 - 2
        } + self.checked_row_offset;

        // check if row cleared
        let checked_row_start_index = usize::from(checked_row) * 10;
        if self.tiles[checked_row_start_index..checked_row_start_index + 10].all() {
            // move tiles down
            let moved_tiles = if checked_row > 0 {
                checked_row * 10
            } else {
                246 // bug from base game: top row clear causes 256 tiles to be moved;
                    // indices 246..256 can be ignored as tiles 256..266 are never read in 1-player mode
            };
            self.tiles.copy_within(0..usize::from(moved_tiles), 10);
            self.tiles[..10].fill(false);

            self.cleared_lines += 1;
        }

        self.checked_row_offset += 1;
        if self.checked_row_offset == 4 {
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
        }

        self.cleared_lines = 0;
        self.play_state = PlayState::Wait2UntilSpawnNextTetrimino;
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

        let new_piece_x;
        if input.get(Input::Right) {
            new_piece_x = self.current_piece_x + 1;
        } else if input.get(Input::Left) {
            new_piece_x = self.current_piece_x - 1;
        } else {
            return;
        }

        if !self.try_set_piece_and_position(self.current_piece, new_piece_x, self.current_piece_y) {
            self.shift_autorepeat = 16;
        }
    }

    fn try_set_piece_and_position(&mut self, piece: Piece, x: i8, y: i8) -> bool {
        for (tile_offset_x, tile_offset_y) in piece.get_tile_offsets() {
            let tile_x = x + tile_offset_x;
            let tile_y = y + tile_offset_y;
            if tile_x < 0 || tile_x >= 10 || tile_y >= 20 {
                return false;
            }

            let y = i16::from(*tile_offset_x)
                + i16::from(*tile_offset_y * 10)
                + i16::from(y as u8) * 10
                + i16::from(x as u8);
            if self.get_tile((y as u8 % 10).into(), (y as u8 / 10).into()) {
                return false;
            }
        }

        self.current_piece = piece;
        self.current_piece_x = x;
        self.current_piece_y = y;
        true
    }

    fn rotate_tetrimino(&mut self, input: Input) {
        let new_piece_rotation;
        let pressed_input = input.difference(self.previous_input);
        if pressed_input.get(Input::A) {
            new_piece_rotation = self.current_piece.get_clockwise_rotation();
        } else if pressed_input.get(Input::B) {
            new_piece_rotation = self.current_piece.get_counterclockwise_rotation();
        } else {
            return;
        }

        self.try_set_piece_and_position(
            new_piece_rotation,
            self.current_piece_x,
            self.current_piece_y,
        );
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
                if !self.try_set_piece_and_position(
                    self.current_piece,
                    self.current_piece_x,
                    self.current_piece_y + 1,
                ) {
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
            if !self.try_set_piece_and_position(
                self.current_piece,
                self.current_piece_x,
                self.current_piece_y + 1,
            ) {
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
