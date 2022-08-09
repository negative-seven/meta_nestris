use crate::{
    input::{Button, Input},
    random::Random,
};

#[derive(Clone, Eq, PartialEq)]
pub struct State {
    pub do_nmi: bool,
    pub dead: bool,
    pub nmi_wait_point: i32,
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
    pub allegro: u8,
    pub spawn_id: u8,
    pub render_mode: u8,
    pub autorepeat_y: u8,
    pub current_piece: u8,
    pub next_piece: u8,
    pub vram_row: u8,
    pub pending_garbage: u8,
    pub lines: u8,
    pub lines_high: u8,
    pub play_state: u8,
    pub autorepeat_x: u8,
    pub general_counter: u8,
    pub playfield: [u8; 0x110],
    pub drop_speed: u8,
    pub level_number: u8,
    pub hold_down_points: u8,
    pub game_mode: u8,
    pub line_index: u8,
    pub curtain_row: u8,
    pub completed_lines: u8,
    pub game_type: u8,
    pub completed_row: [u8; 4],
    pub row_y: u8,
    pub frame_counter: u8,
    pub reset: u8,
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
            nmi_wait_point: -3,
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
            allegro: 0,
            spawn_id: 0,
            render_mode: 3,
            autorepeat_y: 0,
            current_piece: 0,
            next_piece: 0,
            vram_row: 0,
            pending_garbage: 0,
            lines: 0,
            lines_high: 0,
            play_state: 0,
            autorepeat_x: 0,
            general_counter: 0,
            playfield: [0xef; 0x110],
            drop_speed: 0,
            level_number: 0,
            hold_down_points: 0,
            game_mode: 4,
            line_index: 0,
            curtain_row: 0,
            completed_lines: 0,
            game_type: 0,
            completed_row: [0; 4],
            row_y: 0,
            reset: 0,
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
        }
    }

    pub fn step(&mut self, input: &Input) -> Option<State> {
        if self.dead {
            return None;
        }

        if self.do_nmi {
            self.nmi();
        }

        if self.nmi_wait_point == 2 {
            self.nmi_wait_point = 0;
            self.init_playfield_if_type_b();
            self.game_mode_state += 1;
            if self.nmi_wait_point != 0 {
                self.previous_input = input.clone();
                return None;
            }
        }

        loop {
            if self.nmi_wait_point < 0 {
                self.reset_vector();
                self.nmi_wait_point += 1;
                break;
            }

            if self.nmi_wait_point == 0 {
                let a = self.branch_on_game_mode(input);
                if self.dead {
                    return None;
                }
                if self.nmi_wait_point == 0 && a == self.game_mode_state {
                    break;
                }
                if self.nmi_wait_point != 0 {
                    break;
                }
            }

            if self.nmi_wait_point == 1 {
                self.pause_loop(input);
                if self.nmi_wait_point == 1 {
                    break;
                }
            }

            if self.nmi_wait_point == 2 {
                break;
            }

            if self.nmi_wait_point == 3 {
                self.init_playfield_if_type_b();
                if self.nmi_wait_point == 3 {
                    break;
                }
            }
        }

        self.previous_input = input.clone();
        None
    }

    fn reset_vector(&mut self) {
        if self.nmi_wait_point != -5 {
            if self.nmi_wait_point > -5 {
                self.init_ram();
            }
            return;
        }

        self.reset += 1;
        self.init_ram();
    }

    fn init_ram(&mut self) {
        self.random = Random::new();
        self.game_mode_state = 0;
        self.game_mode = 0;
        return;
    }

    fn nmi(&mut self) {
        self.render();
        self.frame_counter = u8::wrapping_add(self.frame_counter, 1);
        self.random.step();
    }

    fn branch_on_game_mode(&mut self, input: &Input) -> u8 {
        match self.game_mode {
            0 => self.legal_screen(input),
            1 => self.title_screen(input),
            2 => self.game_type_menu(input),
            3 => self.level_menu(input),
            4 => self.play_and_ending_high_score(input),
            _ => todo!("game mode {}", self.game_mode),
        }
    }

    fn legal_screen(&mut self, input: &Input) -> u8 {
        self.render_mode = 0;
        if self.legal_screen_nmi_timer < 264 {
            self.legal_screen_nmi_timer += 1;
            self.general_counter = 0xff;
            self.do_nmi = self.legal_screen_nmi_timer != 2;
            return 0;
        }

        let pressed_input = input.difference(&self.previous_input);
        if pressed_input.states.data[0] != 0x10 && self.general_counter != 0 {
            self.general_counter -= 1;
            return 0;
        }

        self.game_mode += 1;
        self.title_screen_nmi_timer = 0;
        return 0x10;
    }

    fn title_screen(&mut self, input: &Input) -> u8 {
        self.render_mode = 0;
        if self.title_screen_nmi_timer < 5 {
            self.title_screen_nmi_timer += 1;
            return 0;
        }

        loop {
            let pressed_input = input.difference(&self.previous_input);
            if pressed_input.states.data[0] == 0x10 {
                break;
            }

            return 0;
        }

        self.game_mode += 1;
        self.game_type_menu_nmi_timer = 0;
        return 0;
    }

    fn game_type_menu(&mut self, input: &Input) -> u8 {
        self.render_mode = 1;

        if self.game_type_menu_nmi_timer < 3 {
            self.game_type_menu_nmi_timer += 1;
            return 0;
        }

        loop {
            let pressed_input = input.difference(&self.previous_input);
            if pressed_input.states.data[0] == 1 {
                self.game_type = 1;
            } else if pressed_input.states.data[0] == 2 {
                self.game_type = 0;
            } else if pressed_input.states.data[0] == 0x10 {
                self.game_mode += 1;
                self.level_menu_nmi_timer = 0;
                return 0;
            } else if pressed_input.states.data[0] == 0x40 {
                self.game_mode -= 1;
                self.title_screen_nmi_timer = 0;
                return 0;
            }

            return 0;
        }
    }

    fn level_menu(&mut self, input: &Input) -> u8 {
        self.render_mode = 1;

        if self.level_menu_nmi_timer < 4 {
            self.level_menu_nmi_timer += 1;
            self.do_nmi = self.level_menu_nmi_timer != 1;
            self.original_y = 0;
            self.drop_speed = 0;
            self.start_level %= 10;
            return 0;
        }

        loop {
            self.selecting_level_or_height = self.original_y;
            self.level_menu_handle_level_height_navigation(input);
            self.original_y = self.selecting_level_or_height;

            let pressed_input = input.difference(&self.previous_input);
            if pressed_input.states.data[0] == 0x10 {
                if input.states.data[0] == 0x90 {
                    self.start_level += 10;
                }
                self.game_mode_state = 0;
                self.game_mode += 1;
                return 0;
            }

            if pressed_input.states.data[0] == 0x40 {
                self.game_type_menu_nmi_timer = 0;
                self.game_mode -= 1;
                return 0;
            }

            self.random.choose_random_holes();
            return 0;
        }
    }

    fn level_menu_handle_level_height_navigation(&mut self, input: &Input) {
        let pressed_input = input.difference(&self.previous_input);

        if pressed_input.states.data[0] == 1 {
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

        if pressed_input.states.data[0] == 2 {
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

        if pressed_input.states.data[0] == 4 {
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

        if pressed_input.states.data[0] == 8 {
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
            if pressed_input.states.data[0] == 0x80 {
                self.selecting_level_or_height ^= 1;
            }
        }
    }

    fn play_and_ending_high_score(&mut self, input: &Input) -> u8 {
        match self.game_mode_state {
            0 => self.init_game_background(),
            1 => self.init_game_state(),
            2 => self.update_counters_and_non_player_state(),
            3 => self.handle_game_over(),
            4 => self.update_player1(input),
            5 => self.update_player2(),
            6 => self.check_for_reset_key_combo(input),
            7 => self.start_button_handling(input),
            8 => self.vblank_then_run_state2(),
            _ => panic!("invalid game mode state"),
        }
    }

    fn init_game_background(&mut self) -> u8 {
        if self.init_game_background_nmi_timer < 3 {
            self.init_game_background_nmi_timer += 1;
            return 0;
        }

        self.play_state = 1;
        self.level_number = self.start_level;
        self.game_mode_state += 1;
        0 // player2_startLevel
    }

    fn init_game_state(&mut self) -> u8 {
        self.tetrimino_x = 5;
        self.tetrimino_y = 0;
        self.vram_row = 0;
        self.fall_timer = 0;
        self.pending_garbage = 0;
        self.score = 0;
        self.score_high = 0;
        self.score_higher = 0;
        self.lines = 0;
        self.allegro = 0;
        self.spawn_id = 0;
        self.render_mode = 3;
        self.autorepeat_y = 0xa0;
        self.current_piece = self.choose_next_tetrimino();
        self.random.step();
        self.next_piece = self.choose_next_tetrimino();
        if self.game_type != 0 {
            self.lines = 0x25;
        }
        self.nmi_wait_point = 2;
        0xff
    }

    fn choose_next_tetrimino(&mut self) -> u8 {
        let piece = self.random.next_piece();
        return piece as u8;
    }

    fn update_counters_and_non_player_state(&mut self) -> u8 {
        self.fall_timer += 1;
        self.game_mode_state += 1;
        0 // 0 or 1
    }

    fn handle_game_over(&mut self) -> u8 {
        self.game_mode_state += 1;
        1
    }

    fn update_player1(&mut self, input: &Input) -> u8 {
        self.make_player1_active();
        self.branch_on_play_state_player1(input);
        self.game_mode_state += 1;
        0 // TODO: unsure, complicated
    }

    fn update_player2(&mut self) -> u8 {
        self.game_mode_state += 1;
        0 // TODO: unsure, complicated
    }

    fn check_for_reset_key_combo(&mut self, input: &Input) -> u8 {
        self.game_mode_state += 1;
        input.states.data[0]
    }

    fn start_button_handling(&mut self, input: &Input) -> u8 {
        let pressed_input = input.difference(&self.previous_input);

        if self.game_mode == 5 && pressed_input.states.data[0] == 0x10 {
            self.game_mode = 1;
            self.game_mode_state += 1;
            return 1;
        }

        if self.render_mode == 3 && pressed_input.get(Button::Start) && self.play_state != 10 {
            self.render_mode = 0;
            self.nmi_wait_point = 1;
            return 0;
        }

        self.game_mode_state += 1;
        0
    }

    fn pause_loop(&mut self, input: &Input) {
        let pressed_input = input.difference(&self.previous_input);
        if !(pressed_input.states.data[0] == 0x10) {
            return;
        }

        self.vram_row = 0;
        self.render_mode = 3;
        self.game_mode_state += 1;
        self.nmi_wait_point = 0;
    }

    fn vblank_then_run_state2(&mut self) -> u8 {
        self.game_mode_state = 2;
        2
    }

    fn make_player1_active(&self) {
        return;
    }

    fn branch_on_play_state_player1(&mut self, input: &Input) {
        match self.play_state {
            0 => self.unassign_orientation_id(),
            1 => self.player_controls_active_tetrimino(input),
            2 => self.lock_tetrimino(),
            3 => self.check_for_completed_rows(),
            4 => (),
            5 => self.update_lines_and_statistics(),
            6 => self.b_type_goal_check(),
            7 => self.receive_garbage(),
            8 => self.spawn_next_tetrimino(),
            9 => (),
            10 => self.update_game_over_curtain(),
            11 => self.increment_play_state(),
            _ => panic!("invalid play state"),
        }
    }

    fn unassign_orientation_id(&mut self) {
        self.current_piece = 0x13;
    }

    fn player_controls_active_tetrimino(&mut self, input: &Input) {
        self.shift_tetrimino(input);
        self.rotate_tetrimino(input);
        self.drop_tetrimino(input);
    }

    fn lock_tetrimino(&mut self) {
        if !self.is_position_valid() {
            self.play_state = 0xa;
            self.curtain_row = 0xf0;
            self.dead = true;
            return;
        }

        if self.vram_row >= 32 {
            self.general_counter = self.tetrimino_y << 1;
            let carry = if self.general_counter as u16 + (self.tetrimino_y << 3) as u16 >= 0x100 {
                1
            } else {
                0
            };
            self.general_counter += (self.tetrimino_y << 3) + self.tetrimino_x + carry;
            let general_counter2 = self.current_piece << 2;
            let mut x = general_counter2 + (self.current_piece << 3);
            let mut general_counter3 = 4;

            loop {
                let general_counter4 = Self::ORIENTATION_TABLE[x as usize] << 1;
                let selecting_level_or_height = u8::wrapping_add(
                    general_counter4,
                    u8::wrapping_add(general_counter4 << 2, self.general_counter),
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
                general_counter3 -= 1;
                if general_counter3 == 0 {
                    break;
                }
            }

            self.line_index = 0;
            self.update_playfield();
            self.play_state += 1;
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

        self.general_counter = general_counter2 << 1;
        self.general_counter += general_counter2 << 3;
        let mut y = self.general_counter;
        let mut x = 10;

        loop {
            if self.playfield[y as usize] == 0xef {
                self.completed_row[self.line_index as usize] = 0;
                self.increment_line_index();
                return;
            }
            y += 1;
            x -= 1;

            if x == 0 {
                break;
            }
        }

        self.completed_lines += 1;
        self.completed_row[self.line_index as usize] = general_counter2;

        let mut y = u8::wrapping_sub(self.general_counter, 1);
        loop {
            self.playfield[y as usize + 10] = self.playfield[y as usize];
            y = u8::wrapping_sub(y, 1);
            if y == 0xff {
                break;
            }
        }

        let mut y = 0;
        loop {
            self.playfield[y] = 0xef;
            y += 1;
            if y == 0xa {
                break;
            }
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
        self.play_state += 1;
        if self.completed_lines == 0 {
            self.play_state += 1;
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

        let mut x = self.completed_lines;

        loop {
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

                // lsr/ror hell
                self.general_counter >>= 4;
                self.general_counter |= general_counter2 << 4;
                if self.level_number < self.general_counter {
                    self.level_number += 1;
                }
            }

            x -= 1;
            if x == 0 {
                break;
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

    fn b_type_goal_check(&mut self) {
        if self.game_type == 0 {
            self.play_state += 1;
            return;
        }

        if self.lines != 0 {
            self.play_state += 1;
            return;
        }

        todo!(); // may not be needed
    }

    fn receive_garbage(&mut self) {
        self.play_state += 1;
        return;
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

    fn update_game_over_curtain(&self) {
        return;
    }

    fn increment_play_state(&mut self) {
        self.play_state += 1;
    }

    fn shift_tetrimino(&mut self, input: &Input) {
        let original_y = self.tetrimino_x;
        if input.get(Button::Down) {
            return;
        }

        let pressed_input = input.difference(&self.previous_input);
        if pressed_input.states.data[0] & 0x3 == 0 {
            if input.states.data[0] & 3 == 0 {
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

        if input.get(Button::Right) {
            self.tetrimino_x += 1;
            if !self.is_position_valid() {
                self.tetrimino_x = original_y;
                self.autorepeat_x = 16;
            }
            return;
        } else if input.get(Button::Left) {
            self.tetrimino_x = u8::wrapping_sub(self.tetrimino_x, 1);
            if !self.is_position_valid() {
                self.tetrimino_x = original_y;
                self.autorepeat_x = 16;
            }
            return;
        }
    }

    fn is_position_valid(&mut self) -> bool {
        self.general_counter = self.tetrimino_y << 1;
        self.general_counter += u8::wrapping_add(self.tetrimino_y << 3, self.tetrimino_x);
        let general_counter2 = self.current_piece << 2;
        let mut x = general_counter2 + (self.current_piece << 3);
        let mut general_counter3 = 4;

        loop {
            if u8::wrapping_add(Self::ORIENTATION_TABLE[x as usize], self.tetrimino_y + 2) >= 0x16 {
                self.general_counter = 0xff;
                return false;
            }

            let general_counter4 = Self::ORIENTATION_TABLE[x as usize] << 1;
            let selecting_level_or_height = u8::wrapping_add(
                Self::ORIENTATION_TABLE[x as usize] << 3,
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
            general_counter3 -= 1;

            if general_counter3 == 0 {
                break;
            }
        }

        self.general_counter = 0;

        true
    }

    fn rotate_tetrimino(&mut self, input: &Input) {
        let original_y = self.current_piece;
        let mut x = self.current_piece << 1;
        let pressed_input = input.difference(&self.previous_input);
        if pressed_input.get(Button::A) {
            x += 1;
            self.current_piece = Self::ROTATION_TABLE[x as usize];
            if !self.is_position_valid() {
                self.current_piece = original_y;
                return;
            }
            return;
        }
        if pressed_input.get(Button::B) {
            self.current_piece = Self::ROTATION_TABLE[x as usize];
            if !self.is_position_valid() {
                self.current_piece = original_y;
                return;
            }
        }
    }

    fn drop_tetrimino(&mut self, input: &Input) {
        let new_input = input.difference(&self.previous_input);
        if self.autorepeat_y >= 0x80 {
            if !new_input.get(Button::Down) {
                self.autorepeat_y = u8::wrapping_add(self.autorepeat_y, 1);
                return;
            }
            self.autorepeat_y = 0;
        }
        if self.autorepeat_y == 0 {
            if input.get(Button::Left) || input.get(Button::Right) {
                self.lookup_drop_speed();
                return;
            }
            if new_input.states.data[0] & 0xf == 4 {
                self.autorepeat_y = 1;
            }
            self.lookup_drop_speed();
            return;
        }

        if !(input.get(Button::Down)
            && !input.get(Button::Left)
            && !input.get(Button::Right)
            && !input.get(Button::Up))
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
        return;
    }

    fn lookup_drop_speed(&mut self) {
        let mut a = 1;
        let x = self.level_number;
        if x < 0x1d {
            a = Self::FRAMES_PER_DROP_TABLE[x as usize];
        }
        self.drop_speed = a;
        if self.fall_timer < self.drop_speed {
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
        return;
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
        self.general_counter = self.level_number + 1;
        loop {
            self.score += Self::POINTS_TABLE[(self.completed_lines << 1) as usize];
            if self.score >= 0xa0 {
                self.score = u8::wrapping_add(self.score, 0x60);
                self.score_high += 1;
            }
            self.score_high += Self::POINTS_TABLE[(self.completed_lines << 1) as usize + 1];
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
            self.general_counter -= 1;
            if self.general_counter == 0 {
                break;
            }
        }
        self.completed_lines = 0;
        self.play_state += 1;
    }

    fn render(&mut self) {
        match self.render_mode {
            0 => self.render_mode_legal_and_title_screen(),
            1 => self.render_mode_menu_screens(),
            2 => todo!(),
            3 => self.render_mode_play_and_demo(),
            4 => todo!(),
            _ => panic!("invalid render mode"),
        }
    }

    fn render_mode_menu_screens(&mut self) {
        return;
    }

    fn render_mode_play_and_demo(&mut self) {
        if self.play_state == 4 {
            self.update_line_clearing_animation();
            self.vram_row = 0;
        } else {
            self.copy_playfield_row_to_vram();
            self.copy_playfield_row_to_vram();
            self.copy_playfield_row_to_vram();
            self.copy_playfield_row_to_vram();
        }
    }

    fn update_line_clearing_animation(&mut self) {
        if self.frame_counter & 0x3 != 0 {
            return;
        }

        self.row_y += 1;
        if self.row_y >= 5 {
            self.play_state += 1;
        }
    }

    fn copy_playfield_row_to_vram(&mut self) {
        if self.vram_row >= 0x15 {
            return;
        }

        self.vram_row += 1;
        if self.vram_row >= 0x14 {
            self.vram_row = 0x20;
        }
    }

    fn render_mode_legal_and_title_screen(&mut self) {
        return;
    }

    fn init_playfield_if_type_b(&mut self) {
        if self.nmi_wait_point == 0 {
            if self.game_type == 0 {
                return;
            }

            self.general_counter = 0xc;
        }
        loop {
            if self.general_counter == 0 {
                break;
            }

            let general_counter2 = 0x14 - self.general_counter;
            self.vram_row = 0;
            let mut general_counter3 = 9;
            loop {
                self.random.step();
                let general_counter4 = Self::RNG_TABLE[(self.random.get_value() & 7) as usize];
                let x = general_counter2;
                let y = Self::MULT_BY10_TABLE[x as usize] + general_counter3;
                self.playfield[y as usize] = general_counter4;
                if general_counter3 == 0 {
                    break;
                }
                general_counter3 -= 1;
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

            self.nmi_wait_point = 3;
            return;
        }
        self.nmi_wait_point = 0;

        let x = self.start_height;
        let mut y = Self::TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE[x as usize];
        loop {
            self.playfield[y as usize] = 0xef;
            y = u8::wrapping_sub(y, 1);
            if y == 0xff {
                break;
            }
        }

        return;
    }
}
