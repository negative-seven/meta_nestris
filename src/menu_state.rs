use crate::{
    game_type::GameType, gameplay_state::GameplayState, input::Input, menu_mode::MenuMode,
    piece::Piece, random::Random,
};
use bitvec::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub struct MenuState {
    pub nmi_on: bool,
    pub previous_input: Input,
    pub random: Random,
    pub menu_mode: MenuMode,
    pub game_type: GameType,
    pub frame_counter: u8,
    pub selecting_height: bool,
    pub selected_level: u8,
    pub selected_height: u8,
    pub copyright_skip_timer: u8,
    pub delay_timer: u16,
    pub change_to_gameplay_state: bool,
    pub initialize_tiles_for_b_type: bool,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub tiles: BitArr!(for 0x100),
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
            nmi_on: false,
            previous_input: Input::new(),
            frame_counter: 3,
            random,
            menu_mode: MenuMode::CopyrightScreen,
            game_type: GameType::A,
            selected_level: 0,
            selecting_height: false,
            selected_height: 0,
            copyright_skip_timer: 0xff,
            delay_timer: 268,
            change_to_gameplay_state: false,
            initialize_tiles_for_b_type: false,
            current_piece: Piece::TUp,
            next_piece: Piece::TUp,
            tiles: BitArray::ZERO,
        }
    }

    pub fn step(&mut self, input: Input) -> Option<GameplayState> {
        if self.nmi_on {
            self.frame_counter = (self.frame_counter + 1) % 4;
            self.random.step();
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
            if self.delay_timer == 0 {
                self.nmi_on = true;
            } else {
                self.previous_input = input.clone();
                return None;
            }
        }

        if self.initialize_tiles_for_b_type {
            self.initialize_type_b_tiles();
            self.initialize_tiles_for_b_type = false;
            self.previous_input = input.clone();
            return None;
        }

        if self.change_to_gameplay_state {
            return Some(GameplayState::new(
                &self.random,
                self.frame_counter,
                self.previous_input,
                self.game_type,
                self.selected_level,
                &self.tiles,
                self.current_piece,
                self.next_piece,
            ));
        }

        self.step_main_logic(input);
        self.previous_input = input.clone();

        None
    }

    pub fn get_tile(&self, x: usize, y: usize) -> bool {
        self.tiles[y * 10 + x]
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: bool) {
        self.tiles.set(y * 10 + x, tile);
    }

    fn step_main_logic(&mut self, input: Input) {
        match self.menu_mode {
            MenuMode::CopyrightScreen => self.step_legal_screen(input),
            MenuMode::TitleScreen => self.step_title_screen(input),
            MenuMode::GameTypeSelect => self.step_game_type_menu(input),
            MenuMode::LevelSelect => self.step_level_menu(input),
            MenuMode::InitializingGame => self.step_init_game_state(),
        }
    }

    fn step_legal_screen(&mut self, input: Input) {
        self.nmi_on = true;

        let pressed_input = input.difference(self.previous_input);
        if pressed_input != Input::Start && self.copyright_skip_timer != 0 {
            self.copyright_skip_timer -= 1;
            return;
        }

        self.menu_mode = MenuMode::TitleScreen;
        self.delay_timer = 5;
    }

    fn step_title_screen(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        if pressed_input == Input::Start {
            self.menu_mode = MenuMode::GameTypeSelect;
            self.delay_timer = 4;
        }
    }

    fn step_game_type_menu(&mut self, input: Input) {
        let pressed_input = input.difference(self.previous_input);
        match pressed_input {
            Input::Left => {
                self.game_type = GameType::A;
            }
            Input::Right => {
                self.game_type = GameType::B;
            }
            Input::Start => {
                self.menu_mode = MenuMode::LevelSelect;
                self.delay_timer = 5;
                self.selecting_height = false;
                self.selected_level %= 10;
                self.nmi_on = false;
                for _ in 0..4 {
                    self.random.step();
                }
            }
            Input::B => {
                self.menu_mode = MenuMode::TitleScreen;
                self.delay_timer = 6;
            }
            _ => (),
        }
    }

    fn step_level_menu(&mut self, input: Input) {
        self.nmi_on = true;

        let pressed_input = input.difference(self.previous_input);

        if self.selecting_height {
            let new_height = i8::try_from(self.selected_height).unwrap()
                + match pressed_input {
                    Input::Right => 1,
                    Input::Left => -1,
                    Input::Down => 3,
                    Input::Up => -3,
                    _ => 0,
                };

            if new_height >= 0 && new_height < 6 {
                self.selected_height = new_height.try_into().unwrap();
            }
        } else {
            let new_level = i8::try_from(self.selected_level).unwrap()
                + match pressed_input {
                    Input::Right => 1,
                    Input::Left => -1,
                    Input::Down => 5,
                    Input::Up => -5,
                    _ => 0,
                };

            if new_level >= 0 && new_level < 10 {
                self.selected_level = new_level.try_into().unwrap();
            }
        }

        if pressed_input == Input::A && self.game_type == GameType::B {
            self.selecting_height ^= true;
        }

        if pressed_input == Input::Start {
            if input == Input::Start | Input::A {
                self.selected_level += 10;
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

    fn step_init_game_state(&mut self) {
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
                self.initialize_tiles_for_b_type = true;
            }
        }
        self.nmi_on = false;
        self.change_to_gameplay_state = true;
    }

    fn initialize_type_b_tiles(&mut self) {
        for general_counter2 in 8..20 {
            self.frame_counter = (self.frame_counter + 1) % 4;
            self.random.step();

            for general_counter3 in (0..10).rev() {
                self.random.step();
                let general_counter4 = Self::RNG_TABLE[(self.random.get_value() % 8) as usize];
                self.set_tile(general_counter3, general_counter2.into(), general_counter4);
            }

            loop {
                self.random.step();
                if self.random.get_value() % 16 < 10 {
                    break;
                }
            }

            let general_counter5 = self.random.get_value() % 16;
            let y = general_counter5 + general_counter2 * 10;
            self.set_tile((y % 10).into(), (y / 10).into(), false);
        }

        for y in 0..Self::TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE[usize::from(self.selected_height)]
        {
            for x in 0..10 {
                self.set_tile(x, y.into(), false);
            }
        }
        self.set_tile(
            0,
            Self::TYPE_BBLANK_INIT_COUNT_BY_HEIGHT_TABLE[usize::from(self.selected_height)].into(),
            false,
        ); // behavior from the base game: leftmost tile of top row is always empty
        self.delay_timer = 12;
    }
}
