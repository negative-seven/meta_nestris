use std::convert::TryInto;

use crate::{
    game_mode_state::GameModeState, game_type::GameType, input::Input, modifier::Modifier,
    piece::Piece, play_state::PlayState, random::Random,
};
use bitvec::prelude::*;

/// A de facto gameplay state; i.e. a state where the playfield is present.
///
/// The `MODIFIER` const generic specifies game modifiers - see [`Modifier`] for
/// supported modifiers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameplayState<const MODIFIER: Modifier> {
    // each field is listed with its equivalent from the base game
    pub dead: bool,   // $68 == #10, once true never changes back to false
    pub paused: bool, // true if execution is in loop at $a3c4
    pub game_mode_state: GameModeState, // $a7
    pub play_state: PlayState, // $68
    pub checked_row_offset: u8, // $aa in routine at $99a2, but counts up instead of down
    pub update_lines_delay: u8, // $72
    pub previous_input: Input, // $f7 on previous frame
    pub random: Random, // $17-$1a
    pub frame_counter: u8, // $b1
    pub rendering_delay: u8, // $69
    pub cleared_lines: u8, // $76
    pub current_piece_x: i8, // $60
    pub current_piece_y: i8, // $61
    pub hold_down_points: u8, // $6f
    pub fall_timer: u8, // $65
    pub drop_autorepeat: i8, // $6e
    pub shift_autorepeat: u8, // $66
    pub game_type: GameType, // $c1
    pub tiles: BitArr!(for 0x100), // $400-$4ff
    pub current_piece: Piece, // $62
    pub next_piece: Piece, // $bf
    pub score: u32,   // $73-$75
    pub level: u8,    // $64
    pub line_count: u16, // $70
    pub play_state_delay: u8, // timer which corresponds to frames where $68 == 7 or $68 == 8
}

impl GameplayState<{ Modifier::empty() }> {
    /// Creates a `GameplayState` with an "empty" [`Modifier`].
    ///
    /// Equivalent to `GameplayState::<{ Modifier::empty()
    /// }>::new_with_modifier`.
    #[must_use]
    pub fn new(
        random: &Random,
        frame_counter: u8,
        previous_input: Input,
        game_type: GameType,
        level: u8,
        b_type_height: u8,
    ) -> Self {
        Self::new_with_modifier(
            random,
            frame_counter,
            previous_input,
            game_type,
            level,
            b_type_height,
        )
    }
}

impl<const MODIFIER: Modifier> GameplayState<MODIFIER> {
    /// Creates a `GameplayState` with a [`Modifier`].
    ///
    /// Example:
    /// ```
    /// use meta_nestris::{GameType, GameplayState, Input, Modifier, Random};
    ///
    /// const MODIFIER: Modifier = Modifier {
    ///     uncapped_score: true,
    ///     ..Modifier::empty()
    /// };
    ///
    /// // both equivalent:
    /// let state_a = GameplayState::<MODIFIER>::new_with_modifier(
    ///     &Random::new(),
    ///     0,
    ///     Input::None,
    ///     GameType::A,
    ///     19,
    ///     0,
    /// );
    /// let state_b: GameplayState<MODIFIER> =
    ///     GameplayState::new_with_modifier(&Random::new(), 0, Input::None, GameType::A, 19, 0);
    /// ```
    #[must_use]
    pub fn new_with_modifier(
        random: &Random,
        frame_counter: u8,
        previous_input: Input,
        game_type: GameType,
        level: u8,
        b_type_height: u8,
    ) -> Self {
        let mut state = Self {
            dead: false,
            previous_input,
            score: 0,
            random: random.clone(),
            current_piece_x: 5,
            current_piece_y: 0,
            fall_timer: 0,
            game_mode_state: GameModeState::HandleGameplay,
            drop_autorepeat: -96,
            current_piece: Piece::None,
            next_piece: Piece::None,
            rendering_delay: 0,
            line_count: match game_type {
                GameType::A => 0,
                GameType::B => 25,
            },
            play_state: PlayState::MoveTetrimino,
            shift_autorepeat: 15,
            tiles: BitArray::ZERO,
            level,
            hold_down_points: 0,
            checked_row_offset: 0,
            cleared_lines: 0,
            game_type,
            update_lines_delay: 0,
            frame_counter,
            paused: false,
            play_state_delay: 0,
        };

        state.current_piece = state.random.get_piece();
        state.random.cycle();
        state.next_piece = state.random.get_piece();

        if game_type == GameType::B {
            state.initialize_type_b_tiles(b_type_height);
        }

        state
    }

    #[must_use]
    pub fn get_tile(&self, x: usize, y: usize) -> bool {
        self.tiles[y * 10 + x]
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: bool) {
        self.tiles.set(y * 10 + x, tile);
    }

    fn get_automatic_drop_delay(&self) -> u8 {
        const LEVEL_DELAYS: [u8; 29] = [
            48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2,
        ];

        if usize::from(self.level) < LEVEL_DELAYS.len() {
            LEVEL_DELAYS[self.level as usize]
        } else {
            1
        }
    }

    /// Steps to the next state.
    pub fn step(&mut self, input: Input) {
        if self.dead {
            return;
        }

        self.frame_counter = (self.frame_counter + 1) % 4;
        self.random.cycle();

        if self.paused {
            if input.difference(self.previous_input) == Input::Start {
                // unpause
                self.rendering_delay = 0;
                self.game_mode_state = GameModeState::Unpause;
                self.paused = false;
            }
        } else if self.play_state == PlayState::DoNothing {
            if self.frame_counter == 1 {
                self.update_lines_delay -= 1;
                if self.update_lines_delay == 0 {
                    self.play_state = PlayState::UpdateLinesAndStatistics;
                }
            }
        } else if self.rendering_delay < 5 {
            self.rendering_delay += 1;
        }

        if !self.paused {
            self.step_main_logic(input);
        }

        self.previous_input = input;
    }

    fn step_main_logic(&mut self, input: Input) {
        if self.game_mode_state == GameModeState::HandleGameplay {
            self.fall_timer += 1;
            if self.play_state_delay > 0 {
                self.play_state_delay -= 1;
            } else {
                self.run_play_state_operation(input);
            }
            self.game_mode_state = GameModeState::HandleStartButton;

            if self.dead || input == Input::Right | Input::Left | Input::Down
            // bug from base game - holding right, left and down causes the frame to end early
            {
                return;
            }
        }

        if self.game_mode_state == GameModeState::HandleStartButton
            && input.difference(self.previous_input).get(Input::Start)
        {
            self.paused = true;
        }

        self.game_mode_state = GameModeState::HandleGameplay;
    }

    fn run_play_state_operation(&mut self, input: Input) {
        match self.play_state {
            PlayState::MoveTetrimino => {
                self.try_shift_piece(input);
                self.try_rotate_piece(input);
                self.try_drop_piece(input);
            }
            PlayState::LockTetrimino => self.lock_tetrimino(),
            PlayState::CheckForCompletedRows => self.check_if_row_completed(),
            PlayState::DoNothing => (),
            PlayState::UpdateLinesAndStatistics => self.update_score_and_line_count(),
            PlayState::SpawnNextTetrimino => self.spawn_next_piece(),
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

        if self.rendering_delay >= 5 {
            for (tile_offset_x, tile_offset_y) in self.current_piece.get_tile_offsets() {
                let x = i16::from(self.current_piece_x) + i16::from(*tile_offset_x);
                let y = i16::from(self.current_piece_y) + i16::from(*tile_offset_y);
                let offset = (y * 10 + x) as u8;
                self.set_tile((offset % 10).into(), (offset / 10).into(), true);
            }

            self.checked_row_offset = 0;
            self.update_rendered_row_counter();
            self.play_state = PlayState::CheckForCompletedRows;
        }
    }

    fn check_if_row_completed(&mut self) {
        if self.rendering_delay < 5 {
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
                246 // bug from base game: top row clear causes 256 tiles to be
                    // moved; indices 246..256 can be ignored as tiles 256..266
                    // are never read in 1-player mode
            };
            self.tiles.copy_within(0..usize::from(moved_tiles), 10);
            self.tiles[..10].fill(false);

            self.cleared_lines += 1;
        }

        self.checked_row_offset += 1;
        if self.checked_row_offset == 4 {
            self.rendering_delay = 0;
            self.update_lines_delay = 5;
            self.play_state = PlayState::DoNothing;
            if self.cleared_lines == 0 {
                self.play_state = PlayState::UpdateLinesAndStatistics;
            }
        }
    }

    fn update_score_and_line_count(&mut self) {
        const BASE_LINE_CLEAR_POINTS: [u16; 5] = [0, 40, 100, 300, 1200];

        fn to_bcd(number: u8) -> u8 {
            number + 6 * (number / 10)
        }

        fn from_bcd(number: u8) -> u8 {
            (number / 16) * 10 + (number % 16)
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
            let low_digits = from_bcd(to_bcd((self.score % 100) as u8) + self.hold_down_points - 1);
            self.score = self.score / 100 * 100;
            self.score += u32::from(low_digits);
            if low_digits >= 100 {
                self.score = self.score / 10 * 10;
            }
        }
        self.hold_down_points = 0;

        self.score += u32::from(BASE_LINE_CLEAR_POINTS[self.cleared_lines as usize])
            * u32::from(self.level + 1);

        if !MODIFIER.uncapped_score {
            self.score = self.score.min(999_999);
        }

        self.cleared_lines = 0;

        self.play_state = PlayState::SpawnNextTetrimino;
        self.play_state_delay = 2;
    }

    fn spawn_next_piece(&mut self) {
        if self.rendering_delay < 5 {
            return;
        }
        self.fall_timer = 0;
        self.current_piece_y = 0;
        self.play_state = PlayState::MoveTetrimino;
        self.current_piece_x = 5;
        self.current_piece = self.next_piece;
        self.next_piece = self.random.get_piece();
        self.drop_autorepeat = 0;
    }

    fn try_shift_piece(&mut self, input: Input) {
        if input.get(Input::Down) {
            return;
        }
        if !input.get(Input::Left) && !input.get(Input::Right) {
            return;
        }

        let pressed_input = input.difference(self.previous_input);
        if pressed_input.get(Input::Left) || pressed_input.get(Input::Right) {
            // new left/right press; try to shift piece immediately, but set autorepeat
            // timer to high value. note that this can be triggered on two
            // consecutive frames with left/right followed by left+right - this allows for
            // two movements to the right on two consecutive frames
            self.shift_autorepeat = 15;
        } else if self.shift_autorepeat == 0 {
            // autorepeat timer elapsed; try to shift piece
            self.shift_autorepeat = 5;
        } else {
            // autorepeat timer not elapsed; decrement and don't try to shift piece
            self.shift_autorepeat -= 1;
            return;
        }

        let new_piece_x = if input.get(Input::Right) {
            self.current_piece_x + 1 // right held or left + right held
        } else {
            self.current_piece_x - 1 // left held
        };

        if !self.try_set_piece_and_position(self.current_piece, new_piece_x, self.current_piece_y) {
            self.shift_autorepeat = 0;
        }
    }

    fn try_set_piece_and_position(&mut self, piece: Piece, x: i8, y: i8) -> bool {
        for (tile_offset_x, tile_offset_y) in piece.get_tile_offsets() {
            let signed_tile_x = x + tile_offset_x;
            let signed_tile_y = y + tile_offset_y;
            if !(0..10).contains(&signed_tile_x) || !(..20).contains(&signed_tile_y) {
                return false;
            }

            let mut tile_x;
            let mut tile_y;
            if signed_tile_y >= 0 {
                // both coordinates are guaranteed to be non-negative here due to earlier checks
                tile_x = signed_tile_x.try_into().unwrap();
                tile_y = signed_tile_y.try_into().unwrap();
            } else {
                // signed_tile_y < 0 causes strange indexing due to 8-bit integer overflow
                // e.g.: tile 9 of row -1 ends up being indexed as tile 5 of row 25
                tile_x = signed_tile_x as usize + 6; // signed_tile_x is guaranteed to be non-negative here due to earlier check
                tile_y = (signed_tile_y + 25) as usize; // guaranteed signed_tile_y >= -2 because y >= 0 and tile_offset_y >= -2
                if tile_x >= 10 {
                    tile_x -= 10;
                    tile_y += 1;
                }
            }

            if self.get_tile(tile_x, tile_y) {
                return false;
            }
        }

        self.current_piece = piece;
        self.current_piece_x = x;
        self.current_piece_y = y;
        true
    }

    fn try_rotate_piece(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        let new_piece_rotation = if pressed_input.get(Input::A) {
            self.current_piece.get_clockwise_rotation()
        } else if pressed_input.get(Input::B) {
            self.current_piece.get_counterclockwise_rotation()
        } else {
            return;
        };

        self.try_set_piece_and_position(
            new_piece_rotation,
            self.current_piece_x,
            self.current_piece_y,
        );
    }

    fn try_drop_piece(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);

        if self.drop_autorepeat < 0 {
            // handle initial piece delay
            if pressed_input.get(Input::Down) {
                // end initial piece delay
                self.drop_autorepeat = 0;
            } else {
                // increment delay timer and leave
                self.drop_autorepeat += 1;
                return;
            }
        }

        let mut drop = false;

        if self.drop_autorepeat == 0 {
            // pushdown not in progress
            if pressed_input.get(Input::Down)
                && !input.get(Input::Left)
                && !input.get(Input::Right)
                && !pressed_input.get(Input::Up)
            {
                // begin manual pushdown
                // note: the first drop takes a frame longer than subsequent drops
                self.drop_autorepeat = 1;
            }
        } else {
            // pushdown in progress
            if input.get(Input::Down)
                && !input.get(Input::Left)
                && !input.get(Input::Right)
                && !input.get(Input::Up)
            {
                // continue manual pushdown
                self.drop_autorepeat += 1;
                if self.drop_autorepeat >= 3 {
                    // manual drop
                    self.drop_autorepeat = 1;
                    self.hold_down_points += 1;

                    drop = true;
                }
            } else {
                // cancel manual pushdown
                self.drop_autorepeat = 0;
                self.hold_down_points = 0;
            }
        }

        if !drop && self.fall_timer >= self.get_automatic_drop_delay() {
            // automatic periodic drop
            drop = true;
        }

        if drop {
            self.fall_timer = 0;
            if !self.try_set_piece_and_position(
                self.current_piece,
                self.current_piece_x,
                self.current_piece_y + 1,
            ) {
                self.play_state = PlayState::LockTetrimino;
                self.update_rendered_row_counter();
            }
        }
    }

    fn update_rendered_row_counter(&mut self) {
        let highest_row_to_update = if self.current_piece_y >= 2 {
            self.current_piece_y as u8 - 2
        } else {
            0
        };
        self.rendering_delay = u8::min(self.rendering_delay * 4, highest_row_to_update) / 4;
    }

    fn initialize_type_b_tiles(&mut self, height_index: u8) {
        const B_TYPE_HEIGHTS: [u8; 6] = [20, 17, 15, 12, 10, 8];
        const B_TYPE_RNG_TABLE: [bool; 8] = [false, true, false, true, true, true, false, false];

        for y in 8..20 {
            self.random.cycle();

            // place tiles randomly
            for x in (0..10).rev() {
                self.random.cycle();
                self.set_tile(
                    x,
                    y,
                    B_TYPE_RNG_TABLE[(self.random.get_value() % 8) as usize],
                );
            }

            // guarantee a hole in the row
            self.random.cycle_do_while(|v| v % 16 >= 10);
            let x = usize::from(self.random.get_value() % 16);
            self.set_tile(x, y, false);
        }

        // behavior from the base game: one additional tile (leftmost tile of the
        // highest garbage row) is also cleared
        let tiles_to_clear = usize::from(B_TYPE_HEIGHTS[usize::from(height_index)]) * 10 + 1;
        self.tiles[..tiles_to_clear].fill(false);
    }
}
