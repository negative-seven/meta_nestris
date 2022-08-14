use crate::{gameplay_state::GameplayState, input::Input, menu_mode::MenuMode, random::Random};

#[derive(Clone, Eq, PartialEq)]
pub struct MenuState {
    pub do_nmi: bool,
    pub previous_input: Input,
    pub random: Random,
    pub game_mode: MenuMode,
    pub game_type: u8,
    pub frame_counter: u8,
    pub start_level: u8,
    pub original_y: u8,
    pub selecting_level_or_height: u8,
    pub start_height: u8,
    pub timeout_counter: u8,
    pub delay_timer: u16,
    pub reset_complete: bool,
    pub to_gameplay_state: bool,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            do_nmi: false,
            previous_input: Input::new(),
            frame_counter: 3,
            random: Random::new(),
            game_mode: MenuMode::CopyrightScreen,
            game_type: 0,
            start_level: 0,
            original_y: 0,
            selecting_level_or_height: 0,
            start_height: 0,
            timeout_counter: 0xff,
            delay_timer: 268,
            reset_complete: false,
            to_gameplay_state: false,
        }
    }

    pub fn step(&mut self, input: Input) -> Option<GameplayState> {
        if self.do_nmi {
            self.frame_counter = (self.frame_counter + 1) % 4;
            self.random.step();
        }

        if !self.reset_complete {
            for _ in 0..263 {
                self.random.step();
            }
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

        self.branch_on_game_mode(input);
        self.previous_input = input.clone();

        return if self.to_gameplay_state {
            Some(GameplayState::from_menu_state(self))
        } else {
            None
        };
    }

    fn branch_on_game_mode(&mut self, input: Input) {
        match self.game_mode {
            MenuMode::CopyrightScreen => self.legal_screen(input),
            MenuMode::TitleScreen => self.title_screen(input),
            MenuMode::GameTypeSelect => self.game_type_menu(input),
            MenuMode::LevelSelect => self.level_menu(input),
        }
    }

    fn legal_screen(&mut self, input: Input) {
        self.do_nmi = true;

        let pressed_input = input.difference(self.previous_input);
        if pressed_input != Input::Start && self.timeout_counter != 0 {
            self.timeout_counter -= 1;
            return;
        }

        self.game_mode = MenuMode::TitleScreen;
        self.delay_timer = 5;
    }

    fn title_screen(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Start {
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
            self.do_nmi = false;
            for _ in 0..4 {
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
}
