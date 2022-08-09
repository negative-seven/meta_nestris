use crate::{input::Input, random::Random};

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
    pub current_piece: u8,
    pub next_piece: u8,
    pub vram_row: u8,
    pub lines: u8,
    pub lines_high: u8,
    pub play_state: u8,
    pub autorepeat_x: u8,
    pub general_counter: u8,
    pub playfield: [u8; 0x110],
    pub level_number: u8,
    pub hold_down_points: u8,
    pub game_mode: u8,
    pub line_index: u8,
    pub completed_lines: u8,
    pub game_type: u8,
    pub completed_row: [u8; 4],
    pub row_y: u8,
    pub frame_counter: u8,
    pub legal_screen_nmi_timer: u16,
    pub legal_screen_skip_timer: u8,
    pub title_screen_nmi_timer: u8,
    pub game_type_menu_nmi_timer: u8,
    pub level_menu_nmi_timer: u8,
    pub start_level: u8,
    pub original_y: u8,
    pub selecting_level_or_height: u8,
    pub init_game_background_nmi_timer: u8,
    pub start_height: u8,
    pub reset_nmi_timer: u8,
    pub paused: bool,
    pub initializing_playfield: bool,
    pub start_init_playfield: bool,
}

impl State {
    const ORIENTATION_TABLE: [u8; 240] = [
        0x00, 0x7B, 0xFF, 0x00, 0x7B, 0x00, 0x00, 0x7B, 0x01, 0xFF, 0x7B, 0x00, 0xFF, 0x7B, 0x00,
        0x00, 0x7B, 0x00, 0x00, 0x7B, 0x01, 0x01, 0x7B, 0x00, 0x00, 0x7B, 0xFF, 0x00, 0x7B, 0x00,
        0x00, 0x7B, 0x01, 0x01, 0x7B, 0x00, 0xFF, 0x7B, 0x00, 0x00, 0x7B, 0xFF, 0x00, 0x7B, 0x00,
        0x01, 0x7B, 0x00, 0xFF, 0x7D, 0x00, 0x00, 0x7D, 0x00, 0x01, 0x7D, 0xFF, 0x01, 0x7D, 0x00,
        0xFF, 0x7D, 0xFF, 0x00, 0x7D, 0xFF, 0x00, 0x7D, 0x00, 0x00, 0x7D, 0x01, 0xFF, 0x7D, 0x00,
        0xFF, 0x7D, 0x01, 0x00, 0x7D, 0x00, 0x01, 0x7D, 0x00, 0x00, 0x7D, 0xFF, 0x00, 0x7D, 0x00,
        0x00, 0x7D, 0x01, 0x01, 0x7D, 0x01, 0x00, 0x7C, 0xFF, 0x00, 0x7C, 0x00, 0x01, 0x7C, 0x00,
        0x01, 0x7C, 0x01, 0xFF, 0x7C, 0x01, 0x00, 0x7C, 0x00, 0x00, 0x7C, 0x01, 0x01, 0x7C, 0x00,
        0x00, 0x7B, 0xFF, 0x00, 0x7B, 0x00, 0x01, 0x7B, 0xFF, 0x01, 0x7B, 0x00, 0x00, 0x7D, 0x00,
        0x00, 0x7D, 0x01, 0x01, 0x7D, 0xFF, 0x01, 0x7D, 0x00, 0xFF, 0x7D, 0x00, 0x00, 0x7D, 0x00,
        0x00, 0x7D, 0x01, 0x01, 0x7D, 0x01, 0xFF, 0x7C, 0x00, 0x00, 0x7C, 0x00, 0x01, 0x7C, 0x00,
        0x01, 0x7C, 0x01, 0x00, 0x7C, 0xFF, 0x00, 0x7C, 0x00, 0x00, 0x7C, 0x01, 0x01, 0x7C, 0xFF,
        0xFF, 0x7C, 0xFF, 0xFF, 0x7C, 0x00, 0x00, 0x7C, 0x00, 0x01, 0x7C, 0x00, 0xFF, 0x7C, 0x01,
        0x00, 0x7C, 0xFF, 0x00, 0x7C, 0x00, 0x00, 0x7C, 0x01, 0xFE, 0x7B, 0x00, 0xFF, 0x7B, 0x00,
        0x00, 0x7B, 0x00, 0x01, 0x7B, 0x00, 0x00, 0x7B, 0xFE, 0x00, 0x7B, 0xFF, 0x00, 0x7B, 0x00,
        0x00, 0x7B, 0x01, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
    ];
    const ROTATION_TABLE: [u8; 38] = [
        3, 1, 0, 2, 1, 3, 2, 0, 7, 5, 4, 6, 5, 7, 6, 4, 9, 9, 8, 8, 0xa, 0xa, 0xc, 0xc, 0xb, 0xb,
        0x10, 0xe, 0xd, 0xf, 0xe, 0x10, 0xf, 0xd, 0x12, 0x12, 0x11, 0x11,
    ];
    const FRAMES_PER_DROP_TABLE: [u8; 30] = [
        0x30, 0x2B, 0x26, 0x21, 0x1C, 0x17, 0x12, 0x0D, 0x08, 0x06, 0x05, 0x05, 0x05, 0x04, 0x04,
        0x04, 0x03, 0x03, 0x03, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x01,
    ];
    const SPAWN_ORIENTATION_FROM_ORIENTATION: [u8; 19] = [
        0x02, 0x02, 0x02, 0x02, 0x07, 0x07, 0x07, 0x07, 0x08, 0x08, 0x0A, 0x0B, 0x0B, 0x0E, 0x0E,
        0x0E, 0x0E, 0x12, 0x12,
    ];
    const POINTS_TABLE: [u8; 10] = [0x00, 0x00, 0x40, 0, 0, 1, 0, 3, 0, 0x12];
    const TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE: [u8; 6] = [0xc8, 0xaa, 0x96, 0x78, 0x64, 0x50];
    const RNG_TABLE: [u8; 8] = [0xef, 0x7b, 0xef, 0x7c, 0x7d, 0x7d, 0xef, 0xef];
    const MULT_BY10_TABLE: [u8; 20] = [
        0x00, 0x0A, 0x14, 0x1E, 0x28, 0x32, 0x3C, 0x46, 0x50, 0x5A, 0x64, 0x6E, 0x78, 0x82, 0x8C,
        0x96, 0xA0, 0xAA, 0xB4, 0xBE,
    ];

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
            current_piece: 0,
            next_piece: 0,
            vram_row: 0,
            lines: 0,
            lines_high: 0,
            play_state: 0,
            autorepeat_x: 0,
            general_counter: 0,
            playfield: [0xef; 0x110],
            level_number: 0,
            hold_down_points: 0,
            game_mode: 4,
            line_index: 0,
            completed_lines: 0,
            game_type: 0,
            completed_row: [0; 4],
            row_y: 0,
            reset_nmi_timer: 0,
            legal_screen_nmi_timer: 0,
            legal_screen_skip_timer: 0,
            title_screen_nmi_timer: 0,
            game_type_menu_nmi_timer: 0,
            start_level: 0,
            level_menu_nmi_timer: 0,
            original_y: 0,
            selecting_level_or_height: 0,
            init_game_background_nmi_timer: 0,
            start_height: 0,
            paused: false,
            initializing_playfield: false,
            start_init_playfield: false,
        }
    }

    pub fn step(&mut self, input: Input) {
        if self.dead {
            return;
        }

        if self.do_nmi {
            self.render();
            self.frame_counter = u8::wrapping_add(self.frame_counter, 1);
            self.random.step();
        }

        if self.reset_nmi_timer < 3 {
            self.random = Random::new();
            self.game_mode_state = 0;
            self.game_mode = 0;
            self.reset_nmi_timer += 1;
            self.previous_input = input.clone();
            return;
        } else if self.paused {
            self.pause_loop(input);
            if self.paused {
                self.previous_input = input.clone();
                return;
            }
        } else if self.initializing_playfield {
            self.init_playfield_if_type_b();
            if self.initializing_playfield {
                self.previous_input = input.clone();
                return;
            }
        } else if self.start_init_playfield {
            self.start_init_playfield = false;
            self.init_playfield_if_type_b();
            self.game_mode_state = 2;
            if self.start_init_playfield {
                self.previous_input = input.clone();
                return;
            }
        }

        loop {
            let a = self.branch_on_game_mode(input);
            if self.dead || self.start_init_playfield || a || self.paused {
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
        self.render_playfield = false;
        if self.legal_screen_nmi_timer < 264 {
            self.legal_screen_nmi_timer += 1;
            self.general_counter = 0xff;
            self.do_nmi = self.legal_screen_nmi_timer != 2;
            return true;
        }

        let pressed_input = input.difference(self.previous_input);
        if pressed_input != 0x10 && self.general_counter != 0 {
            self.general_counter -= 1;
            return true;
        }

        self.game_mode = 1;
        self.title_screen_nmi_timer = 0;
        false
    }

    fn title_screen(&mut self, input: Input) {
        self.render_playfield = false;
        if self.title_screen_nmi_timer < 5 {
            self.title_screen_nmi_timer += 1;
            return;
        }

        let pressed_input = input.difference(self.previous_input);
        if pressed_input != Input::Start {
            return;
        }

        self.game_mode = 2;
        self.game_type_menu_nmi_timer = 0;
    }

    fn game_type_menu(&mut self, input: Input) {
        self.render_playfield = false;

        if self.game_type_menu_nmi_timer < 3 {
            self.game_type_menu_nmi_timer += 1;
            return;
        }

        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Right {
            self.game_type = 1;
        } else if pressed_input == Input::Left {
            self.game_type = 0;
        } else if pressed_input == Input::Start {
            self.game_mode = 3;
            self.level_menu_nmi_timer = 0;
            return;
        } else if pressed_input == Input::B {
            self.game_mode = 1;
            self.title_screen_nmi_timer = 0;
            return;
        }

        return;
    }

    fn level_menu(&mut self, input: Input) {
        self.render_playfield = false;

        if self.level_menu_nmi_timer < 4 {
            self.level_menu_nmi_timer += 1;
            self.do_nmi = self.level_menu_nmi_timer != 1;
            self.original_y = 0;
            self.start_level %= 10;
            return;
        }

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
            return;
        }

        if pressed_input == Input::B {
            self.game_type_menu_nmi_timer = 0;
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
            0 => self.init_game_background(),
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
                self.game_mode_state = 8;
                false
            }
            8 => {
                self.game_mode_state = 2;
                true
            }
            _ => panic!("invalid game mode state"),
        }
    }

    fn init_game_background(&mut self) -> bool {
        if self.init_game_background_nmi_timer < 3 {
            self.init_game_background_nmi_timer += 1;
            return true;
        }

        self.play_state = 1;
        self.level_number = self.start_level;
        self.game_mode_state = 1;
        false
    }

    fn init_game_state(&mut self) {
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
        self.current_piece = self.choose_next_tetrimino();
        self.random.step();
        self.next_piece = self.choose_next_tetrimino();
        if self.game_type != 0 {
            self.lines = 0x25;
        }
        self.start_init_playfield = true;
    }

    fn choose_next_tetrimino(&mut self) -> u8 {
        let piece = self.random.next_piece();
        return piece as u8;
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
            0 => {
                self.current_piece = 0x13;
            }
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
            9 => (),
            10 => (),
            11 => {
                self.play_state = 12;
            }
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
            self.general_counter = self.tetrimino_y * 2;
            let carry = if self.general_counter as u16 + (self.tetrimino_y * 8) as u16 >= 0x100 {
                1
            } else {
                0
            };
            self.general_counter += (self.tetrimino_y * 8) + self.tetrimino_x + carry;
            let mut x = self.current_piece * 12;

            for _ in 0..4 {
                let general_counter4 = u8::wrapping_mul(Self::ORIENTATION_TABLE[x as usize], 2);
                let selecting_level_or_height = u8::wrapping_add(
                    general_counter4,
                    u8::wrapping_add(u8::wrapping_mul(general_counter4, 4), self.general_counter),
                );
                x += 1;
                let general_counter5 = Self::ORIENTATION_TABLE[x as usize];
                x += 1;
                let y = u8::wrapping_add(
                    Self::ORIENTATION_TABLE[x as usize],
                    selecting_level_or_height,
                );
                self.playfield[y as usize] = general_counter5;
                x += 1;
            }

            self.line_index = 0;
            self.update_playfield();
            self.play_state = 3;
        }
    }

    fn check_for_completed_rows(&mut self) {
        if self.vram_row < 0x20 {
            return;
        }

        let general_counter2 = if self.tetrimino_y < 2 {
            0
        } else {
            self.tetrimino_y - 2
        } + self.line_index;

        self.general_counter = general_counter2 * 2;
        self.general_counter += general_counter2 * 8;
        let mut y = self.general_counter;

        for _ in 0..10 {
            if self.playfield[y as usize] == 0xef {
                self.completed_row[self.line_index as usize] = 0;
                self.increment_line_index();
                return;
            }
            y += 1;
        }

        self.completed_lines += 1;
        self.completed_row[self.line_index as usize] = general_counter2;

        let mut y = u8::wrapping_sub(self.general_counter, 1);
        loop {
            self.playfield[y as usize + 10] = self.playfield[y as usize];
            if y == 0 {
                break;
            }
            y = u8::wrapping_sub(y, 1);
        }

        for y in 0..10 {
            self.playfield[y] = 0xef;
        }

        self.current_piece = 0x13;
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
            self.general_counter = self.completed_lines;
            self.lines = u8::wrapping_sub(self.lines, self.general_counter);
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
                self.general_counter = self.lines;

                self.general_counter /= 16;
                self.general_counter |= general_counter2 * 16;
                if self.level_number < self.general_counter {
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
        if self.vram_row < 0x20 {
            return;
        }
        self.fall_timer = 0;
        self.tetrimino_y = 0;
        self.play_state = 1;
        self.tetrimino_x = 5;
        self.current_piece = Self::SPAWN_ORIENTATION_FROM_ORIENTATION[self.next_piece as usize];
        self.next_piece = self.choose_next_tetrimino();
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
        self.general_counter = self.tetrimino_y * 2;
        self.general_counter += u8::wrapping_add(self.tetrimino_y * 8, self.tetrimino_x);
        let mut x = self.current_piece * 12;

        for _ in 0..4 {
            if u8::wrapping_add(Self::ORIENTATION_TABLE[x as usize], self.tetrimino_y + 2) >= 0x16 {
                self.general_counter = 0xff;
                return false;
            }

            let general_counter4 = u8::wrapping_mul(Self::ORIENTATION_TABLE[x as usize], 2);
            let selecting_level_or_height = u8::wrapping_add(
                u8::wrapping_mul(Self::ORIENTATION_TABLE[x as usize], 8),
                u8::wrapping_add(general_counter4, self.general_counter),
            );
            x += 2;
            let y = u8::wrapping_add(
                Self::ORIENTATION_TABLE[x as usize],
                selecting_level_or_height,
            );
            if self.playfield[y as usize] < 0xef {
                self.general_counter = 0xff;
                return false;
            }

            if u8::wrapping_add(Self::ORIENTATION_TABLE[x as usize], self.tetrimino_x) >= 10 {
                self.general_counter = 0xff;
                return false;
            }
            x += 1;
        }

        self.general_counter = 0;

        true
    }

    fn rotate_tetrimino(&mut self, input: Input) {
        let original_y = self.current_piece;
        let mut x = self.current_piece * 2;
        let pressed_input = input.difference(self.previous_input);
        if pressed_input.get(Input::A) {
            x += 1;
            self.current_piece = Self::ROTATION_TABLE[x as usize];
            if !self.is_position_valid() {
                self.current_piece = original_y;
            }
            return;
        }
        if pressed_input.get(Input::B) {
            self.current_piece = Self::ROTATION_TABLE[x as usize];
            if !self.is_position_valid() {
                self.current_piece = original_y;
                return;
            }
        }
    }

    fn drop_tetrimino(&mut self, input: Input) {
        let new_input = input.difference(self.previous_input);
        if self.autorepeat_y >= 0x80 {
            if !new_input.get(Input::Down) {
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
            if new_input.get(Input::Down)
                && !new_input.get(Input::Left)
                && !new_input.get(Input::Right)
                && !new_input.get(Input::Up)
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
        let mut a = 1;
        let x = self.level_number;
        if x < 0x1d {
            a = Self::FRAMES_PER_DROP_TABLE[x as usize];
        }
        if self.fall_timer < a {
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
        let mut a = u8::wrapping_sub(self.tetrimino_y, 2);
        if a >= 0x80 {
            a = 0;
        }
        if a < self.vram_row {
            self.vram_row = a;
        }
    }

    fn add_line_clear_points(&mut self) {
        self.hold_down_points = 0;
        for _ in 0..self.level_number + 1 {
            self.score += Self::POINTS_TABLE[(self.completed_lines * 2) as usize];
            if self.score >= 0xa0 {
                self.score = u8::wrapping_add(self.score, 0x60);
                self.score_high += 1;
            }
            self.score_high += Self::POINTS_TABLE[(self.completed_lines * 2) as usize + 1];
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
            if self.frame_counter & 0x3 == 0 {
                self.row_y += 1;
                if self.row_y >= 5 {
                    self.play_state = 5;
                }
            }
            self.vram_row = 0;
        } else {
            for _ in 0..4 {
                if self.vram_row >= 0x15 {
                    return;
                }

                self.vram_row += 1;
                if self.vram_row >= 0x14 {
                    self.vram_row = 0x20;
                }
            }
        }
    }

    fn init_playfield_if_type_b(&mut self) {
        if !self.start_init_playfield {
            if self.game_type == 0 {
                return;
            }

            self.general_counter = 0xc;
        }
        if self.general_counter != 0 {
            let general_counter2 = 0x14 - self.general_counter;
            self.vram_row = 0;
            for general_counter3 in (0..10).rev() {
                self.random.step();
                let general_counter4 = Self::RNG_TABLE[(self.random.get_value() & 7) as usize];
                let x = general_counter2;
                let y = Self::MULT_BY10_TABLE[x as usize] + general_counter3;
                self.playfield[y as usize] = general_counter4;
            }

            loop {
                self.random.step();
                if self.random.get_value() & 0xf < 0xa {
                    break;
                }
            }

            let general_counter5 = self.random.get_value() & 0xf;
            let y = general_counter5 + Self::MULT_BY10_TABLE[general_counter2 as usize];
            self.playfield[y as usize] = 0xef;
            self.general_counter -= 1;

            self.start_init_playfield = true;
            self.initializing_playfield = true;
            return;
        }
        self.start_init_playfield = false;
        self.initializing_playfield = false;

        for y in 0..=Self::TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE[self.start_height as usize] {
            self.playfield[y as usize] = 0xef;
        }

        return;
    }
}
