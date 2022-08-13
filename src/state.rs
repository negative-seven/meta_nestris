use crate::{input::Input, piece::Piece, random::Random};

#[derive(Clone, Eq, PartialEq)]
pub struct State {
    pub do_nmi: bool,
    pub dead: bool,
    pub previous_input: Input,
    pub level: i8,
    pub score: u8,
    pub score_high: u8,
    pub score_higher: u8,
    pub random: Random,
    pub tetrimino_x: u8,
    pub tetrimino_y: u8,
    pub fall_timer: u8,
    pub game_mode_state: u8,
    pub render_playfield: bool,
    pub autorepeat_y: u8,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub vram_row: u8,
    pub lines: u8,
    pub lines_high: u8,
    pub play_state: u8,
    pub autorepeat_x: u8,
    pub playfield: [bool; 0x110],
    pub level_number: u8,
    pub hold_down_points: u8,
    pub game_mode: u8,
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

impl State {
    const ORIENTATION_TABLE: [[[i8; 2]; 4]; 19] = [
        [[0, -1], [0, 0], [0, 1], [-1, 0]],
        [[-1, 0], [0, 0], [0, 1], [1, 0]],
        [[0, -1], [0, 0], [0, 1], [1, 0]],
        [[-1, 0], [0, -1], [0, 0], [1, 0]],
        [[-1, 0], [0, 0], [1, -1], [1, 0]],
        [[-1, -1], [0, -1], [0, 0], [0, 1]],
        [[-1, 0], [-1, 1], [0, 0], [1, 0]],
        [[0, -1], [0, 0], [0, 1], [1, 1]],
        [[0, -1], [0, 0], [1, 0], [1, 1]],
        [[-1, 1], [0, 0], [0, 1], [1, 0]],
        [[0, -1], [0, 0], [1, -1], [1, 0]],
        [[0, 0], [0, 1], [1, -1], [1, 0]],
        [[-1, 0], [0, 0], [0, 1], [1, 1]],
        [[-1, 0], [0, 0], [1, 0], [1, 1]],
        [[0, -1], [0, 0], [0, 1], [1, -1]],
        [[-1, -1], [-1, 0], [0, 0], [1, 0]],
        [[-1, 1], [0, -1], [0, 0], [0, 1]],
        [[-2, 0], [-1, 0], [0, 0], [1, 0]],
        [[0, -2], [0, -1], [0, 0], [0, 1]],
    ];
    const ROTATION_TABLE: [Piece; 38] = [
        Piece::TLeft,
        Piece::TRight,
        Piece::TUp,
        Piece::TDown,
        Piece::TRight,
        Piece::TLeft,
        Piece::TDown,
        Piece::TUp,
        Piece::JLeft,
        Piece::JRight,
        Piece::JUp,
        Piece::JDown,
        Piece::JRight,
        Piece::JLeft,
        Piece::JDown,
        Piece::JUp,
        Piece::ZVertical,
        Piece::ZVertical,
        Piece::ZHorizontal,
        Piece::ZHorizontal,
        Piece::O,
        Piece::O,
        Piece::SVertical,
        Piece::SVertical,
        Piece::SHorizontal,
        Piece::SHorizontal,
        Piece::LLeft,
        Piece::LRight,
        Piece::LUp,
        Piece::LDown,
        Piece::LRight,
        Piece::LLeft,
        Piece::LDown,
        Piece::LUp,
        Piece::IHorizontal,
        Piece::IHorizontal,
        Piece::IVertical,
        Piece::IVertical,
    ];
    const FRAMES_PER_DROP_TABLE: [u8; 30] = [
        48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 1,
    ];
    const POINTS_TABLE: [u16; 5] = [0x0, 0x40, 0x100, 0x300, 0x1200];
    const TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE: [u8; 6] = [200, 170, 150, 120, 100, 80];
    const RNG_TABLE: [bool; 8] = [false, true, false, true, true, true, false, false];

    pub fn new() -> Self {
        Self {
            do_nmi: false,
            dead: false,
            previous_input: Input::new(),
            level: 0,
            score: 0,
            score_high: 0,
            score_higher: 0,
            frame_counter: 0,
            random: Random::new(),
            tetrimino_x: 0,
            tetrimino_y: 0,
            game_mode_state: 0,
            fall_timer: 0,
            render_playfield: false,
            autorepeat_y: 0,
            current_piece: Piece::TUp,
            next_piece: Piece::TUp,
            vram_row: 0,
            lines: 0,
            lines_high: 0,
            play_state: 0,
            autorepeat_x: 0,
            playfield: [false; 0x110],
            level_number: 0,
            hold_down_points: 0,
            game_mode: 0,
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
            self.game_mode_state = 2;
            self.previous_input = input.clone();
            return;
        } else if self.init_playfield_dummy {
            self.init_playfield_dummy = false;
            self.game_mode_state = 2;
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
        match self.game_mode {
            0 => self.legal_screen(input),
            1 => {
                self.title_screen(input);
                true
            }
            2 => {
                self.game_type_menu(input);
                true
            }
            3 => {
                self.level_menu(input);
                true
            }
            4 => self.play_and_ending_high_score(input),
            _ => todo!("game mode {}", self.game_mode),
        }
    }

    fn legal_screen(&mut self, input: Input) -> bool {
        self.do_nmi = true;

        let pressed_input = input.difference(self.previous_input);
        if pressed_input != Input::Start && self.timeout_counter != 0 {
            self.timeout_counter -= 1;
            return true;
        }

        self.game_mode = 1;
        self.delay_timer = 5;
        self.render_playfield = false;
        true
    }

    fn title_screen(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input != Input::Start {
            return;
        }

        self.render_playfield = false;
        self.game_mode = 2;
        self.delay_timer = 4;
    }

    fn game_type_menu(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Right {
            self.game_type = 1;
        } else if pressed_input == Input::Left {
            self.game_type = 0;
        } else if pressed_input == Input::Start {
            self.game_mode = 3;
            self.delay_timer = 5;
            self.original_y = 0;
            self.start_level %= 10;
            self.render_playfield = false;
            self.do_nmi = false;
            for _ in 0..4 {
                self.render();
                self.random.step();
            }
            return;
        } else if pressed_input == Input::B {
            self.game_mode = 1;
            self.delay_timer = 6;
            return;
        }
    }

    fn level_menu(&mut self, input: Input) {
        self.do_nmi = true;

        self.selecting_level_or_height = self.original_y;
        self.level_menu_handle_level_height_navigation(input);
        self.original_y = self.selecting_level_or_height;

        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Start {
            if input == Input::Start | Input::A {
                self.start_level += 10;
            }
            self.game_mode_state = 0;
            self.game_mode = 4;
            self.delay_timer = 4;
            return;
        }

        if pressed_input == Input::B {
            self.delay_timer = 4;
            self.game_mode = 2;
            return;
        }

        self.random.choose_random_holes();
    }

    fn level_menu_handle_level_height_navigation(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);

        if pressed_input == Input::Right {
            if self.selecting_level_or_height == 0 {
                if self.start_level != 9 {
                    self.start_level += 1;
                }
            } else {
                if self.start_height != 5 {
                    self.start_height += 1;
                }
            }
        }

        if pressed_input == Input::Left {
            if self.selecting_level_or_height == 0 {
                if self.start_level != 0 {
                    self.start_level -= 1;
                }
            } else {
                if self.start_height != 0 {
                    self.start_height -= 1;
                }
            }
        }

        if pressed_input == Input::Down {
            if self.selecting_level_or_height == 0 {
                if self.start_level < 5 {
                    self.start_level += 5;
                }
            } else {
                if self.start_height < 3 {
                    self.start_height += 3;
                }
            }
        }

        if pressed_input == Input::Up {
            if self.selecting_level_or_height == 0 {
                if self.start_level >= 5 {
                    self.start_level -= 5;
                }
            } else {
                if self.start_height >= 3 {
                    self.start_height -= 3;
                }
            }
        }

        if self.game_type != 0 {
            if pressed_input == Input::A {
                self.selecting_level_or_height ^= 1;
            }
        }
    }

    fn play_and_ending_high_score(&mut self, input: Input) -> bool {
        match self.game_mode_state {
            0 => {
                self.game_mode_state = 1;
                false
            }
            1 => {
                self.init_game_state();
                false
            }
            2 => {
                self.fall_timer += 1;
                self.branch_on_play_state_player1(input);
                self.game_mode_state = 7;
                input == self.game_mode_state
            }
            7 => {
                self.start_button_handling(input);
                self.game_mode_state = 2;
                true
            }
            8 => {
                self.game_mode_state = 2;
                true
            }
            _ => panic!("invalid game mode state"),
        }
    }

    fn init_game_state(&mut self) {
        self.play_state = 1;
        self.level_number = self.start_level;
        self.tetrimino_x = 5;
        self.tetrimino_y = 0;
        self.vram_row = 0;
        self.fall_timer = 0;
        self.score = 0;
        self.score_high = 0;
        self.score_higher = 0;
        self.lines = 0;
        self.render_playfield = true;
        self.autorepeat_y = 0xa0;
        self.current_piece = self.random.next_piece();
        self.random.step();
        self.next_piece = self.random.next_piece();
        if self.game_type != 0 {
            self.lines = 0x25;
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

        if self.game_mode == 5 && pressed_input == Input::Start {
            self.game_mode = 1;
            self.game_mode_state = 8;
        }

        if self.render_playfield && pressed_input.get(Input::Start) && self.play_state != 10 {
            self.render_playfield = false;
            self.paused = true;
        }
    }

    fn pause_loop(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input != Input::Start {
            return;
        }

        self.vram_row = 0;
        self.render_playfield = true;
        self.game_mode_state = 8;
        self.paused = false;
    }

    fn branch_on_play_state_player1(&mut self, input: Input) {
        match self.play_state {
            1 => {
                self.shift_tetrimino(input);
                self.rotate_tetrimino(input);
                self.drop_tetrimino(input);
            }
            2 => self.lock_tetrimino(),
            3 => self.check_for_completed_rows(),
            4 => (),
            5 => self.update_lines_and_statistics(),
            6 => {
                self.play_state = 7;
            }
            7 => {
                self.play_state = 8;
            }
            8 => self.spawn_next_tetrimino(),
            _ => panic!("invalid play state"),
        }
    }

    fn lock_tetrimino(&mut self) {
        if !self.is_position_valid() {
            self.play_state = 10;
            self.dead = true;
            return;
        }

        if self.vram_row >= 32 {
            let center_tile_offset = self.tetrimino_y * 10 + self.tetrimino_x; // assumption: self.tetrimino_y < 26, resulting in no carry

            for tile_index in 0..4 {
                let additional_offset_y_component = Self::ORIENTATION_TABLE
                    [self.current_piece as usize][tile_index as usize][0]
                    * 10;
                let additional_offset_x_component =
                    Self::ORIENTATION_TABLE[self.current_piece as usize][tile_index as usize][1];
                let additional_offset =
                    additional_offset_x_component + additional_offset_y_component;
                self.playfield[u8::wrapping_add(center_tile_offset, additional_offset as u8) as u8
                    as usize] = true;
            }

            self.line_index = 0;
            self.update_playfield();
            self.play_state = 3;
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

        for index in general_counter..general_counter + 10 {
            if !self.playfield[index as usize] {
                self.completed_row[self.line_index as usize] = 0;
                self.increment_line_index();
                return;
            }
        }

        self.completed_lines += 1;
        self.completed_row[self.line_index as usize] = general_counter2;

        let mut y = u8::wrapping_sub(general_counter, 1);
        loop {
            self.playfield[y as usize + 10] = self.playfield[y as usize];
            if y == 0 {
                break;
            }
            y = u8::wrapping_sub(y, 1);
        }

        for y in 0..10 {
            self.playfield[y] = false;
        }

        self.current_piece = Piece::None;
        self.increment_line_index();
    }

    fn increment_line_index(&mut self) {
        self.line_index += 1;
        if self.line_index < 4 {
            return;
        }

        self.vram_row = 0;
        self.row_y = 0;
        self.play_state = 4;
        if self.completed_lines == 0 {
            self.play_state = 5;
        }
    }

    fn update_lines_and_statistics(&mut self) {
        if self.completed_lines == 0 {
            self.add_hold_down_points();
            return;
        }

        if self.game_type != 0 {
            self.lines = u8::wrapping_sub(self.lines, self.completed_lines);
            if self.lines >= 0x80 {
                self.lines = 0;
                self.add_hold_down_points();
                return;
            }

            if self.lines & 0xf < 0xa {
                self.add_hold_down_points();
                return;
            }
            self.lines -= 6;
            self.add_hold_down_points();
            return;
        }

        for _ in 0..self.completed_lines {
            self.lines += 1;
            if self.lines & 0xf >= 0xa {
                self.lines += 6;
                if self.lines & 0xf0 >= 0xa0 {
                    self.lines &= 0xf;
                    self.lines_high += 1;
                }
            }

            if self.lines & 0xf == 0 {
                let general_counter2 = self.lines_high;
                let mut general_counter = self.lines;
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
        if self.hold_down_points < 2 {
            self.add_line_clear_points();
            return;
        }
        self.score += self.hold_down_points - 1;
        if self.score & 0xf >= 0xa {
            self.score += 6;
        }

        if self.score & 0xf0 >= 0xa0 {
            self.score = u8::wrapping_add(self.score & 0xf0, 0x60);
            self.score_high += 1;
        }
        self.add_line_clear_points();
    }

    fn spawn_next_tetrimino(&mut self) {
        if self.vram_row < 32 {
            return;
        }
        self.fall_timer = 0;
        self.tetrimino_y = 0;
        self.play_state = 1;
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
            return;
        } else if input.get(Input::Left) {
            self.tetrimino_x = u8::wrapping_sub(self.tetrimino_x, 1);
            if !self.is_position_valid() {
                self.tetrimino_x = original_y;
                self.autorepeat_x = 16;
            }
            return;
        }
    }

    fn is_position_valid(&mut self) -> bool {
        let center_tile_offset =
            i16::from(u8::wrapping_add(self.tetrimino_y * 10, self.tetrimino_x));

        for x2 in 0..4 {
            if i16::from(Self::ORIENTATION_TABLE[self.current_piece as usize][x2 as usize][0])
                + i16::from(self.tetrimino_y)
                + 2
                >= 0x16
            {
                return false;
            }

            let general_counter4 =
                i16::from(Self::ORIENTATION_TABLE[self.current_piece as usize][x2 as usize][0] * 2);
            let selecting_level_or_height =
                i16::from(Self::ORIENTATION_TABLE[self.current_piece as usize][x2 as usize][0] * 8)
                    + general_counter4
                    + center_tile_offset;
            let y = i16::from(Self::ORIENTATION_TABLE[self.current_piece as usize][x2 as usize][1])
                + selecting_level_or_height;
            if self.playfield[y as u8 as usize] {
                return false;
            }

            if u8::wrapping_add(
                Self::ORIENTATION_TABLE[self.current_piece as usize][x2 as usize][1] as u8,
                self.tetrimino_x,
            ) >= 10
            {
                return false;
            }
        }

        true
    }

    fn rotate_tetrimino(&mut self, input: Input) {
        let original_y = self.current_piece;
        let pressed_input = input.difference(self.previous_input);
        if pressed_input.get(Input::A) {
            self.current_piece = Self::ROTATION_TABLE[(self.current_piece as u8 * 2 + 1) as usize];
            if !self.is_position_valid() {
                self.current_piece = original_y;
            }
            return;
        }
        if pressed_input.get(Input::B) {
            self.current_piece = Self::ROTATION_TABLE[(self.current_piece as u8 * 2) as usize];
            if !self.is_position_valid() {
                self.current_piece = original_y;
                return;
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
        if self.autorepeat_y < 3 {
            self.lookup_drop_speed();
            return;
        }
        self.autorepeat_y = 1;
        self.hold_down_points += 1;

        self.fall_timer = 0;
        let original_y = self.tetrimino_y;
        self.tetrimino_y += 1;
        if self.is_position_valid() {
            return;
        }
        self.tetrimino_y = original_y;
        self.play_state = 2;
        self.update_playfield();
    }

    fn lookup_drop_speed(&mut self) {
        let frames_per_drop = if self.level_number < 0x1d {
            Self::FRAMES_PER_DROP_TABLE[self.level_number as usize]
        } else {
            1
        };
        if self.fall_timer < frames_per_drop {
            return;
        }

        self.fall_timer = 0;
        let original_y = self.tetrimino_y;
        self.tetrimino_y += 1;
        if self.is_position_valid() {
            return;
        }
        self.tetrimino_y = original_y;
        self.play_state = 2;
        self.update_playfield();
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
            self.score += (Self::POINTS_TABLE[self.completed_lines as usize] & 0xff) as u8;
            if self.score >= 0xa0 {
                self.score = u8::wrapping_add(self.score, 0x60);
                self.score_high += 1;
            }
            self.score_high += (Self::POINTS_TABLE[self.completed_lines as usize] >> 8) as u8;
            if self.score_high & 0xf >= 0xa {
                self.score_high += 6;
            }
            if self.score_high & 0xf0 >= 0xa0 {
                self.score_high = u8::wrapping_add(self.score_high, 0x60);
                self.score_higher += 1;
            }
            if self.score_higher & 0xf >= 0xa {
                self.score_higher += 6;
            }
            if self.score_higher & 0xf0 >= 0xa0 {
                self.score = 0x99;
                self.score_high = 0x99;
                self.score_higher = 0x99;
            }
        }
        self.completed_lines = 0;
        self.play_state = 6;
    }

    fn render(&mut self) {
        if !self.render_playfield {
            return;
        }

        if self.play_state == 4 {
            if self.frame_counter == 0 {
                self.row_y += 1;
                if self.row_y >= 5 {
                    self.play_state = 5;
                }
            }
            self.vram_row = 0;
        } else {
            if self.vram_row >= 21 {
                return;
            }

            self.vram_row += 4;
            if self.vram_row >= 20 {
                self.vram_row = 32;
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
                let x = general_counter2;
                let y = x * 10 + general_counter3;
                self.playfield[y as usize] = general_counter4;
            }

            loop {
                self.random.step();
                if self.random.get_value() % 16 < 10 {
                    break;
                }
            }

            let general_counter5 = self.random.get_value() % 16;
            let y = general_counter5 + general_counter2 * 10;
            self.playfield[y as usize] = false;
        }

        for y in 0..=Self::TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE[self.start_height as usize] {
            self.playfield[y as usize] = false;
        }
    }
}
