use crate::{input::Input, modifier::Modifier, GameType, GameplayState, MenuMode, Random};

/// A general state of the game.
///
/// For a reduced state object that excludes menu-related variables, see
/// [`GameplayState`].
///
/// The `MODIFIER` const generic specifies game modifiers - see [`Modifier`] for
/// supported modifiers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State<const MODIFIER: Modifier> {
    pub nmi_on: bool,
    pub delay_timer: u16,
    pub change_to_gameplay_state: bool,
    pub menu_mode: MenuMode,
    pub copyright_skip_timer: u8,
    pub previous_input: Input,
    pub random: Random,
    pub frame_counter: u8,
    pub selecting_height: bool,
    pub game_type: GameType,
    pub selected_level: u8,
    pub selected_height: u8,
    pub gameplay_state: Option<GameplayState<MODIFIER>>,
}

impl State<{ Modifier::empty() }> {
    /// Creates a `State` with an "empty" [`Modifier`].
    ///
    /// Equivalent to `State::<{ Modifier::empty() }>::new_with_modifier`.
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_modifier()
    }
}

impl<const MODIFIER: Modifier> State<MODIFIER> {
    /// Creates a `State` with a [`Modifier`].
    ///
    /// Example:
    /// ```
    /// use meta_nestris::{Modifier, State};
    ///
    /// const MODIFIER: Modifier = Modifier {
    ///     uncapped_score: true,
    ///     ..Modifier::empty()
    /// };
    ///
    /// // both equivalent:
    /// let state_a = State::<MODIFIER>::new_with_modifier();
    /// let state_b: State<MODIFIER> = State::new_with_modifier();
    /// ```
    #[must_use]
    pub fn new_with_modifier() -> Self {
        let mut random = Random::new();
        random.cycle_multiple(263);

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
            gameplay_state: None,
        }
    }

    /// Steps to the next state.
    pub fn step(&mut self, input: Input) {
        if let Some(ref mut gameplay_state) = self.gameplay_state {
            gameplay_state.step(input);
            return;
        }

        if self.nmi_on {
            self.frame_counter = (self.frame_counter + 1) % 4;
            self.random.cycle();
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
            if self.delay_timer == 0 {
                self.nmi_on = true;
            } else {
                self.previous_input = input;
                return;
            }
        }

        if self.change_to_gameplay_state {
            self.random.cycle();

            self.gameplay_state = Some(GameplayState::new_with_modifier(
                &self.random,
                self.frame_counter,
                self.previous_input,
                self.game_type,
                self.selected_level,
                self.selected_height,
            ));
            return;
        }

        self.step_main_logic(input);
        self.previous_input = input;
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
                self.random.cycle_multiple(4);
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

            if (0..6).contains(&new_height) {
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

            if (0..10).contains(&new_level) {
                self.selected_level = new_level.try_into().unwrap();
            }
        }

        if pressed_input == Input::A && self.game_type == GameType::B {
            self.selecting_height ^= true;
        }

        if pressed_input == Input::Start {
            self.selected_level +=
                if MODIFIER.select_adds_20_levels && input == Input::Start | Input::Select {
                    20
                } else if input == Input::Start | Input::A {
                    10
                } else {
                    0
                };
            self.delay_timer = 3;
            self.menu_mode = MenuMode::InitializingGame;
        } else if pressed_input == Input::B {
            self.delay_timer = 4;
            self.menu_mode = MenuMode::GameTypeSelect;
        } else {
            for _ in 0..2 {
                self.random.cycle_do_while(|v| v % 16 >= 10);
            }
        }
    }

    fn step_init_game_state(&mut self) {
        self.frame_counter = (self.frame_counter + 1) % 4;
        match self.game_type {
            GameType::A => {
                self.delay_timer = 1;
            }
            GameType::B => {
                self.delay_timer = 13;
            }
        }
        self.nmi_on = false;
        self.change_to_gameplay_state = true;
    }
}

impl<const MODIFIER: Modifier> Default for State<MODIFIER> {
    fn default() -> Self {
        Self::new_with_modifier()
    }
}
