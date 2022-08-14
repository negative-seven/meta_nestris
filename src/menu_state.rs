use crate::{
    game_type::GameType, gameplay_state::GameplayState, input::Input, menu_mode::MenuMode,
    piece::Piece, random::Random,
};

#[derive(Clone, Eq, PartialEq)]
pub struct MenuState {
    pub do_nmi: bool,
    pub previous_input: Input,
    pub random: Random,
    pub menu_mode: MenuMode,
    pub game_type: GameType,
    pub frame_counter: u8,
    pub start_level: u8,
    pub selecting_level_or_height: u8,
    pub start_height: u8,
    pub timeout_counter: u8,
    pub delay_timer: u16,
    pub to_gameplay_state: bool,
    pub init_playfield: bool,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub playfield: [[bool; 10]; 27],
}

impl MenuState {
    const TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE: [u8; 6] = [20, 17, 15, 12, 10, 8];
    const RNG_TABLE: [bool; 8] = [false, true, false, true, true, true, false, false];

    pub fn new() -> Self {
        let mut random = Random::new();
        for _ in 0..263 {
            random.step();
        }

        Self {
            do_nmi: false,
            previous_input: Input::new(),
            frame_counter: 3,
            random,
            menu_mode: MenuMode::CopyrightScreen,
            game_type: GameType::A,
            start_level: 0,
            selecting_level_or_height: 0,
            start_height: 0,
            timeout_counter: 0xff,
            delay_timer: 268,
            to_gameplay_state: false,
            init_playfield: false,
            current_piece: Piece::TUp,
            next_piece: Piece::TUp,
            playfield: [[false; 10]; 27],
        }
    }

    pub fn step(&mut self, input: Input) -> Option<GameplayState> {
        if self.do_nmi {
            self.frame_counter = (self.frame_counter + 1) % 4;
            self.random.step();
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

        if self.init_playfield {
            self.init_playfield_for_type_b();
            self.init_playfield = false;
            self.previous_input = input.clone();
            return None;
        }

        if self.to_gameplay_state {
            return Some(GameplayState::from_menu_state(self));
        }

        self.branch_on_game_mode(input);
        self.previous_input = input.clone();

        None
    }

    fn branch_on_game_mode(&mut self, input: Input) {
        match self.menu_mode {
            MenuMode::CopyrightScreen => self.legal_screen(input),
            MenuMode::TitleScreen => self.title_screen(input),
            MenuMode::GameTypeSelect => self.game_type_menu(input),
            MenuMode::LevelSelect => self.level_menu(input),
            MenuMode::InitializingGame => self.init_game_state(),
        }
    }

    fn legal_screen(&mut self, input: Input) {
        self.do_nmi = true;

        let pressed_input = input.difference(self.previous_input);
        if pressed_input != Input::Start && self.timeout_counter != 0 {
            self.timeout_counter -= 1;
            return;
        }

        self.menu_mode = MenuMode::TitleScreen;
        self.delay_timer = 5;
    }

    fn title_screen(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Start {
            self.menu_mode = MenuMode::GameTypeSelect;
            self.delay_timer = 4;
        }
    }

    fn game_type_menu(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Right {
            self.game_type = GameType::B;
        } else if pressed_input == Input::Left {
            self.game_type = GameType::A;
        } else if pressed_input == Input::Start {
            self.menu_mode = MenuMode::LevelSelect;
            self.delay_timer = 5;
            self.selecting_level_or_height = 0;
            self.start_level %= 10;
            self.do_nmi = false;
            for _ in 0..4 {
                self.random.step();
            }
        } else if pressed_input == Input::B {
            self.menu_mode = MenuMode::TitleScreen;
            self.delay_timer = 6;
        }
    }

    fn level_menu(&mut self, input: Input) {
        self.do_nmi = true;

        self.level_menu_handle_level_height_navigation(input);

        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Start {
            if input == Input::Start | Input::A {
                self.start_level += 10;
            }
            self.delay_timer = 3;
            self.menu_mode = MenuMode::InitializingGame;
        } else if pressed_input == Input::B {
            self.delay_timer = 4;
            self.menu_mode = MenuMode::GameTypeSelect;
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

        if self.game_type == GameType::B {
            if pressed_input == Input::A {
                self.selecting_level_or_height ^= 1;
            }
        }
    }

    fn init_game_state(&mut self) {
        self.frame_counter = (self.frame_counter + 1) % 4;
        self.random.step();
        self.current_piece = self.random.next_piece();
        self.random.step();
        self.next_piece = self.random.next_piece();
        match self.game_type {
            GameType::A => {
                self.delay_timer = 1;
            }
            GameType::B => {
                self.init_playfield = true;
            }
        }
        self.do_nmi = false;
        self.to_gameplay_state = true;
    }

    fn init_playfield_for_type_b(&mut self) {
        for general_counter2 in 8..20 {
            self.frame_counter = (self.frame_counter + 1) % 4;
            self.random.step();

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
        self.delay_timer = 12;
    }
}
