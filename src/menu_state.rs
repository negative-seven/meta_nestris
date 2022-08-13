use crate::{
    menu_mode::MenuMode, game_mode_state::GameModeState, gameplay_state::GameplayState,
    input::Input, piece::Piece, play_state::PlayState, random::Random,
};

#[derive(Clone, Eq, PartialEq)]
pub struct MenuState {
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
    pub to_gameplay_state: bool,
}

impl MenuState {
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
            to_gameplay_state: false,
        }
    }

    pub fn step(&mut self, input: Input) -> Option<GameplayState> {
        if self.dead {
            return None;
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
                return None;
            }
        }

        if self.paused {
            self.pause_loop(input);
            if self.paused {
                self.previous_input = input.clone();
                return None;
            }
        } else if self.init_playfield {
            self.init_playfield_for_type_b();
            self.game_mode_state = GameModeState::HandleGameplay;
            self.previous_input = input.clone();
            return None;
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

                return if self.to_gameplay_state {
                    Some(GameplayState::from_menu_state(self))
                } else {
                    None
                };
            }
        }
    }

    fn branch_on_game_mode(&mut self, input: Input) -> bool {
        match self.game_mode {
            MenuMode::CopyrightScreen => self.legal_screen(input),
            MenuMode::TitleScreen => {
                self.title_screen(input);
                true
            }
            MenuMode::GameTypeSelect => {
                self.game_type_menu(input);
                true
            }
            MenuMode::LevelSelect => {
                self.level_menu(input);
                true
            }
        }
    }

    fn legal_screen(&mut self, input: Input) -> bool {
        self.do_nmi = true;

        let pressed_input = input.difference(self.previous_input);
        if pressed_input != Input::Start && self.timeout_counter != 0 {
            self.timeout_counter -= 1;
            return true;
        }

        self.game_mode = MenuMode::TitleScreen;
        self.delay_timer = 5;
        self.render_playfield = false;
        true
    }

    fn title_screen(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Start {
            self.render_playfield = false;
            self.game_mode = MenuMode::GameTypeSelect;
            self.delay_timer = 4;
        }
    }

    fn game_type_menu(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Right {
            self.game_type = 1;
        } else if pressed_input == Input::Left {
            self.game_type = 0;
        } else if pressed_input == Input::Start {
            self.game_mode = MenuMode::LevelSelect;
            self.delay_timer = 5;
            self.original_y = 0;
            self.start_level %= 10;
            self.render_playfield = false;
            self.do_nmi = false;
            for _ in 0..4 {
                self.render();
                self.random.step();
            }
        } else if pressed_input == Input::B {
            self.game_mode = MenuMode::TitleScreen;
            self.delay_timer = 6;
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
            self.game_mode_state = GameModeState::InitGameState;
            self.to_gameplay_state = true;
            self.delay_timer = 4;
        } else if pressed_input == Input::B {
            self.delay_timer = 4;
            self.game_mode = MenuMode::GameTypeSelect;
        } else {
            self.random.choose_random_holes();
        }
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

    fn pause_loop(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Start {
            self.vram_row = 0;
            self.render_playfield = true;
            self.game_mode_state = GameModeState::Unpause;
            self.paused = false;
        }
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
