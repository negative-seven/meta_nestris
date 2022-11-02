use bitvec::BitArr;

use crate::{
    game_mode_state::GameModeState, game_type::GameType, gameplay_state::GameplayState,
    input::Input, menu_mode::MenuMode, menu_state::MenuState, modifier::Modifier, piece::Piece,
    play_state::PlayState, random::Random, state::State,
};

pub struct MetaState<const MODIFIER: Modifier> {
    pub frame: u32,
    pub dead: bool,
    pub random: Random,
    pub frame_counter: u8,
    pub rendering_delay: u8,
    pub game_type: GameType,
    pub tiles: BitArr!(for 0x100),
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub score: u32,
    pub level: u8,
    pub line_count: u16,
}

impl MetaState<{ Modifier::empty() }> {
    pub fn new(level: u8, game_type: GameType) -> Self {
        Self::new_with_modifier(level, game_type)
    }
}

impl<const MODIFIER: Modifier> MetaState<MODIFIER> {
    pub fn new_with_modifier(level: u8, game_type: GameType) -> Self {
        let inputs = Self::get_menu_inputs(level, game_type);

        let mut state = State::new_with_modifier();
        for input in inputs.iter() {
            state.step(*input);
        }

        match state {
            State::MenuState(_) => {
                panic!("did not reach gameplay state after menu inputs");
            }
            State::GameplayState(gameplay_state) => {
                return Self::from_gameplay_state_unchecked(
                    gameplay_state,
                    inputs.len().try_into().unwrap(),
                );
            }
        }
    }

    fn from_gameplay_state_unchecked(state: GameplayState<MODIFIER>, frame: u32) -> Self {
        Self {
            frame,
            dead: state.dead,
            random: state.random,
            frame_counter: state.frame_counter,
            rendering_delay: state.rendering_delay,
            game_type: state.game_type,
            tiles: state.tiles,
            current_piece: state.current_piece,
            next_piece: state.next_piece,
            score: state.score,
            level: state.level,
            line_count: state.line_count,
        }
    }

    pub fn to_gameplay_state(&self) -> GameplayState<MODIFIER> {
        GameplayState {
            dead: self.dead,
            paused: false,
            game_mode_state: GameModeState::HandleGameplay,
            play_state: PlayState::MoveTetrimino,
            checked_row_offset: 0,
            update_lines_delay: 0,
            previous_input: Input::None,
            random: self.random.clone(),
            frame_counter: self.frame_counter,
            rendering_delay: self.rendering_delay,
            cleared_lines: 0,
            current_piece_x: 5,
            current_piece_y: 0,
            hold_down_points: 0,
            fall_timer: 0,
            drop_autorepeat: if self.line_count == 0 && self.tiles.not_any() {
                -96 // first piece
            } else {
                0
            },
            shift_autorepeat: 15,
            game_type: self.game_type,
            tiles: self.tiles,
            current_piece: self.current_piece,
            next_piece: self.next_piece,
            score: self.score,
            level: self.level,
            line_count: self.line_count,
            play_state_delay: 0,
        }
    }

    pub fn get_inputs(&self) -> Vec<Input> {
        Self::get_menu_inputs(self.level, self.game_type)
    }

    fn get_menu_inputs(level: u8, game_type: GameType) -> Vec<Input> {
        if level != 0 {
            todo!("level != 0");
        }
        if game_type == GameType::B {
            todo!("game type b");
        }

        let mut menu_state = MenuState::<MODIFIER>::new_with_modifier();
        let mut inputs = Vec::new();

        for target_menu_mode in vec![
            MenuMode::TitleScreen,
            MenuMode::GameTypeSelect,
            MenuMode::LevelSelect,
            MenuMode::InitializingGame,
        ] {
            loop {
                let mut new_menu_state = menu_state.clone();
                new_menu_state.step(Input::Start);
                if new_menu_state.menu_mode != target_menu_mode {
                    menu_state.step(Input::None);
                    inputs.push(Input::None);
                } else {
                    menu_state = new_menu_state;
                    inputs.push(Input::Start);
                    break;
                }
            }
        }

        loop {
            let gameplay_state_option = menu_state.step(Input::None);
            inputs.push(Input::None);
            if let Some(_) = gameplay_state_option {
                return inputs;
            }
        }
    }
}
