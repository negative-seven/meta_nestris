use crate::{
    game_mode_state::GameModeState, input::Input, menu_mode::MenuMode, menu_state::MenuState,
    piece::Piece, play_state::PlayState, random::Random,
};

#[derive(Clone, Eq, PartialEq)]
pub struct GameplayState {
    pub do_nmi: bool,
    pub dead: bool,
    pub previous_input: Input,
    pub level: i8,
    pub score: [u8; 3],
    pub random: Random,
    pub tetrimino_x: u8,
    pub tetrimino_y: u8,
    pub fall_timer: u8,
    pub game_mode_state: GameModeState,
    pub render_playfield: bool,
    pub autorepeat_y: u8,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub vram_row: u8,
    pub lines: [u8; 2],
    pub play_state: PlayState,
    pub autorepeat_x: u8,
    pub playfield: [[bool; 10]; 27],
    pub level_number: u8,
    pub hold_down_points: u8,
    pub game_mode: MenuMode,
    pub line_index: u8,
    pub completed_lines: u8,
    pub game_type: u8,
    pub completed_row: [u8; 4],
    pub row_y: u8,
    pub frame_counter: u8,
    pub start_level: u8,
    pub original_y: u8,
    pub selecting_level_or_height: u8,
    pub start_height: u8,
    pub paused: bool,
    pub init_playfield: bool,
    pub init_playfield_dummy: bool,
    pub timeout_counter: u8,
    pub delay_timer: u16,
    pub reset_complete: bool,
}

impl GameplayState {
    const FRAMES_PER_DROP_TABLE: [u8; 30] = [
        48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 1,
    ];
    const POINTS_TABLE: [u16; 5] = [0x0, 0x40, 0x100, 0x300, 0x1200];
    const TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE: [u8; 6] = [20, 17, 15, 12, 10, 8];
    const RNG_TABLE: [bool; 8] = [false, true, false, true, true, true, false, false];

    pub fn new() -> Self {
        Self {
            do_nmi: false,
            dead: false,
            previous_input: Input::new(),
            level: 0,
            score: [0; 3],
            frame_counter: 0,
            random: Random::new(),
            tetrimino_x: 0,
            tetrimino_y: 0,
            game_mode_state: GameModeState::InitGameState,
            fall_timer: 0,
            render_playfield: false,
            autorepeat_y: 0,
            current_piece: Piece::TUp,
            next_piece: Piece::TUp,
            vram_row: 0,
            lines: [0; 2],
            play_state: PlayState::MoveTetrimino,
            autorepeat_x: 0,
            playfield: [[false; 10]; 27],
            level_number: 0,
            hold_down_points: 0,
            game_mode: MenuMode::CopyrightScreen,
            line_index: 0,
            completed_lines: 0,
            game_type: 0,
            completed_row: [0; 4],
            row_y: 0,
            start_level: 0,
            original_y: 0,
            selecting_level_or_height: 0,
            start_height: 0,
            paused: false,
            init_playfield: false,
            init_playfield_dummy: false,
            timeout_counter: 0,
            delay_timer: 268,
            reset_complete: false,
        }
    }

    pub fn from_menu_state(menu_state: &MenuState) -> Self {
        Self {
            do_nmi: menu_state.do_nmi,
            dead: false,
            previous_input: menu_state.previous_input,
            level: 0,
            score: [0; 3],
            random: menu_state.random.clone(),
            tetrimino_x: 0,
            tetrimino_y: 0,
            fall_timer: 0,
            game_mode_state: GameModeState::InitGameState,
            render_playfield: false,
            autorepeat_y: 0,
            current_piece: Piece::TUp,
            next_piece: Piece::TUp,
            vram_row: 0,
            lines: [0; 2],
            play_state: PlayState::MoveTetrimino,
            autorepeat_x: 0,
            playfield: [[false; 10]; 27],
            level_number: 0,
            hold_down_points: 0,
            game_mode: menu_state.game_mode,
            line_index: 0,
            completed_lines: 0,
            game_type: menu_state.game_type,
            completed_row: [0; 4],
            row_y: 0,
            frame_counter: menu_state.frame_counter,
            start_level: menu_state.start_level,
            original_y: menu_state.original_y,
            selecting_level_or_height: menu_state.selecting_level_or_height,
            start_height: menu_state.start_height,
            paused: false,
            init_playfield: false,
            init_playfield_dummy: false,
            timeout_counter: menu_state.timeout_counter,
            delay_timer: menu_state.delay_timer,
            reset_complete: menu_state.reset_complete,
        }
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

        if !self.reset_complete {
            self.timeout_counter = 0xff;
            for _ in 0..263 {
                self.render();
                self.random.step();
            }
            self.frame_counter = (self.frame_counter + 3) % 4;
            self.reset_complete = true;
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
            if self.delay_timer == 0 {
                self.do_nmi = true;
            } else {
                self.previous_input = input.clone();
                return;
            }
        }

        if self.paused {
            self.pause_loop(input);
            if self.paused {
                self.previous_input = input.clone();
                return;
            }
        } else if self.init_playfield {
            self.init_playfield_for_type_b();
            self.game_mode_state = GameModeState::HandleGameplay;
            self.previous_input = input.clone();
            return;
        } else if self.init_playfield_dummy {
            self.init_playfield_dummy = false;
            self.game_mode_state = GameModeState::HandleGameplay;
        }

        loop {
            let force_end_loop = self.branch_on_game_mode(input);
            if force_end_loop
                || self.dead
                || self.init_playfield
                || self.init_playfield_dummy
                || self.paused
            {
                self.previous_input = input.clone();
                return;
            }
        }
    }

    fn branch_on_game_mode(&mut self, input: Input) -> bool {
        self.play_and_ending_high_score(input)
    }

    fn play_and_ending_high_score(&mut self, input: Input) -> bool {
        match self.game_mode_state {
            GameModeState::InitGameState => {
                self.init_game_state();
                false
            }
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

    fn init_game_state(&mut self) {
        self.play_state = PlayState::MoveTetrimino;
        self.level_number = self.start_level;
        self.tetrimino_x = 5;
        self.tetrimino_y = 0;
        self.vram_row = 0;
        self.fall_timer = 0;
        self.score = [0; 3];
        self.lines[0] = 0;
        self.render_playfield = true;
        self.autorepeat_y = 0xa0;
        self.current_piece = self.random.next_piece();
        self.random.step();
        self.next_piece = self.random.next_piece();
        if self.game_type != 0 {
            self.lines[0] = 0x25;
        }
        if self.game_type == 0 {
            self.init_playfield_dummy = true;
        } else {
            self.do_nmi = false;
            self.init_playfield = true;
        }
    }

    fn start_button_handling(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);

        if self.render_playfield && pressed_input.get(Input::Start) {
            self.render_playfield = false;
            self.paused = true;
        }
    }

    fn pause_loop(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Start {
            self.vram_row = 0;
            self.render_playfield = true;
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
                self.playfield[(offset / 10) as usize][(offset % 10) as usize] = true;
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
            self.tetrimino_y - 2
        } + self.line_index;
        let general_counter = general_counter2 * 10;

        for x in 0..10 {
            if !self.playfield[general_counter2 as usize][x as usize] {
                self.completed_row[self.line_index as usize] = 0;
                self.increment_line_index();
                return;
            }
        }

        self.completed_lines += 1;
        self.completed_row[self.line_index as usize] = general_counter2;

        let mut y = u8::wrapping_sub(general_counter, 1);
        loop {
            self.playfield[(y / 10 + 1) as usize][(y % 10) as usize] =
                self.playfield[(y / 10) as usize][(y % 10) as usize];
            if y == 0 {
                break;
            }
            y = u8::wrapping_sub(y, 1);
        }

        for x in 0..10 {
            self.playfield[0][x] = false;
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
        if self.completed_lines == 0 {
            self.add_hold_down_points();
            return;
        }

        if self.game_type != 0 {
            self.lines[0] = u8::wrapping_sub(self.lines[0], self.completed_lines);
            if self.lines[0] >= 0x80 {
                self.lines[0] = 0;
                self.add_hold_down_points();
                return;
            }

            if self.lines[0] & 0xf < 0xa {
                self.add_hold_down_points();
                return;
            }
            self.lines[0] -= 6;
            self.add_hold_down_points();
            return;
        }

        for _ in 0..self.completed_lines {
            self.lines[0] += 1;
            if self.lines[0] & 0xf >= 0xa {
                self.lines[0] += 6;
                if self.lines[0] & 0xf0 >= 0xa0 {
                    self.lines[0] &= 0xf;
                    self.lines[1] += 1;
                }
            }

            if self.lines[0] & 0xf == 0 {
                let general_counter2 = self.lines[1];
                let mut general_counter = self.lines[0];
                general_counter /= 16;
                general_counter |= general_counter2 * 16;
                if self.level_number < general_counter {
                    self.level_number += 1;
                }
            }
        }
        self.add_hold_down_points();
    }

    // is there a bug? this doesn't seem to update highest byte of score
    fn add_hold_down_points(&mut self) {
        if self.hold_down_points >= 2 {
            self.score[0] += self.hold_down_points - 1;
            if self.score[0] & 0xf >= 0xa {
                self.score[0] += 6;
            }

            if self.score[0] & 0xf0 >= 0xa0 {
                self.score[0] = u8::wrapping_add(self.score[0] & 0xf0, 0x60);
                self.score[1] += 1;
            }
        }
        self.add_line_clear_points();
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
            self.tetrimino_x = u8::wrapping_sub(self.tetrimino_x, 1);
            if !self.is_position_valid() {
                self.tetrimino_x = original_y;
                self.autorepeat_x = 16;
            }
        }
    }

    fn is_position_valid(&mut self) -> bool {
        for (tile_offset_x, tile_offset_y) in self.current_piece.get_tile_offsets() {
            if i16::from(*tile_offset_y) + i16::from(self.tetrimino_y) >= 20 {
                return false;
            }

            let y = i16::from(*tile_offset_x)
                + i16::from(*tile_offset_y * 10)
                + i16::from(u8::wrapping_add(self.tetrimino_y * 10, self.tetrimino_x));
            if self.playfield[(y as u8 / 10) as usize][(y as u8 % 10) as usize] {
                return false;
            }

            if u8::wrapping_add(*tile_offset_x as u8, self.tetrimino_x) >= 10 {
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
        if self.autorepeat_y >= 0x80 {
            if !pressed_input.get(Input::Down) {
                self.autorepeat_y = u8::wrapping_add(self.autorepeat_y, 1);
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
            self.tetrimino_y - 2
        } else {
            0
        };
        self.vram_row = u8::min(self.vram_row, highest_row_to_update);
    }

    fn add_line_clear_points(&mut self) {
        self.hold_down_points = 0;
        for _ in 0..self.level_number + 1 {
            self.score[0] += (Self::POINTS_TABLE[self.completed_lines as usize] & 0xff) as u8;
            if self.score[0] >= 0xa0 {
                self.score[0] = u8::wrapping_add(self.score[0], 0x60);
                self.score[1] += 1;
            }
            self.score[1] += (Self::POINTS_TABLE[self.completed_lines as usize] >> 8) as u8;
            if self.score[1] & 0xf >= 0xa {
                self.score[1] += 6;
            }
            if self.score[1] & 0xf0 >= 0xa0 {
                self.score[1] = u8::wrapping_add(self.score[1], 0x60);
                self.score[2] += 1;
            }
            if self.score[2] & 0xf >= 0xa {
                self.score[2] += 6;
            }
            if self.score[2] & 0xf0 >= 0xa0 {
                self.score = [0x99; 3];
            }
        }
        self.completed_lines = 0;
        self.play_state = PlayState::SkipTo7;
    }

    fn render(&mut self) {
        if !self.render_playfield {
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

    fn init_playfield_for_type_b(&mut self) {
        self.generate_playfield();
        self.render();
        self.frame_counter = (self.frame_counter + 1) % 4;
        self.random.step();

        self.delay_timer = 12;
        self.init_playfield = false;
    }

    fn generate_playfield(&mut self) {
        for general_counter2 in 8..20 {
            self.render();
            self.frame_counter = (self.frame_counter + 1) % 4;
            self.random.step();

            self.vram_row = 0;
            for general_counter3 in (0..10).rev() {
                self.random.step();
                let general_counter4 = Self::RNG_TABLE[(self.random.get_value() % 8) as usize];
                self.playfield[general_counter2 as usize][general_counter3 as usize] =
                    general_counter4;
            }

            loop {
                self.random.step();
                if self.random.get_value() % 16 < 10 {
                    break;
                }
            }

            let general_counter5 = self.random.get_value() % 16;
            let y = general_counter5 + general_counter2 * 10;
            self.playfield[(y / 10) as usize][(y % 10) as usize] = false;
        }

        for y in 0..Self::TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE[self.start_height as usize] {
            for x in 0..10 {
                self.playfield[y as usize][x as usize] = false;
            }
        }
        self.playfield
            [Self::TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE[self.start_height as usize] as usize]
            [0] = false; // behavior from the base game: leftmost tile of top row is always empty
    }
}
