use crate::{
    game_mode_state::GameModeState, game_type::GameType, gameplay_state::GameplayState,
    input::Input, menu_state::MenuState, modifier::Modifier, piece::Piece, play_state::PlayState,
    random::Random, state::State,
};
use bitvec::BitArr;
use std::{collections::BTreeSet, rc::Rc};

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
    pub initial_random_index: u16,
}

impl MetaState<{ Modifier::empty() }> {
    pub fn new(level: u8, game_type: GameType, random_index: u16) -> Self {
        Self::new_with_modifier(level, game_type, random_index)
    }
}

impl<const MODIFIER: Modifier> MetaState<MODIFIER> {
    pub fn new_with_modifier(level: u8, game_type: GameType, random_index: u16) -> Self {
        let inputs = Self::get_menu_inputs(level, game_type, random_index);

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
                    random_index,
                );
            }
        }
    }

    fn from_gameplay_state_unchecked(
        state: GameplayState<MODIFIER>,
        frame: u32,
        initial_random_index: u16,
    ) -> Self {
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
            initial_random_index,
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
        Self::get_menu_inputs(self.level, self.game_type, self.initial_random_index)
    }

    fn get_menu_inputs(level: u8, game_type: GameType, random_index: u16) -> Vec<Input> {
        if level != 0 {
            todo!("level != 0");
        }
        if game_type == GameType::B {
            todo!("game type b");
        }

        #[derive(Clone, Eq, PartialEq)]
        struct Node<const MODIFIER: Modifier> {
            pub menu_state: MenuState<MODIFIER>,
            pub previous_node: Option<Rc<Node<MODIFIER>>>,
            pub frame: u32,
        }

        impl<const MODIFIER: Modifier> PartialOrd for Node<MODIFIER> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match self.frame.partial_cmp(&other.frame) {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match self.menu_state.nmi_on.partial_cmp(&other.menu_state.nmi_on) {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match self
                    .menu_state
                    .delay_timer
                    .partial_cmp(&other.menu_state.delay_timer)
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match self
                    .menu_state
                    .change_to_gameplay_state
                    .partial_cmp(&other.menu_state.change_to_gameplay_state)
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match (self.menu_state.menu_mode as u8)
                    .partial_cmp(&(other.menu_state.menu_mode as u8))
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match self
                    .menu_state
                    .copyright_skip_timer
                    .partial_cmp(&other.menu_state.copyright_skip_timer)
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match self
                    .menu_state
                    .previous_input
                    .partial_cmp(&other.menu_state.previous_input)
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match self
                    .menu_state
                    .random
                    .index
                    .partial_cmp(&other.menu_state.random.index)
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match self
                    .menu_state
                    .frame_counter
                    .partial_cmp(&other.menu_state.frame_counter)
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match self
                    .menu_state
                    .selecting_height
                    .partial_cmp(&other.menu_state.selecting_height)
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match (self.menu_state.game_type as u8)
                    .partial_cmp(&(other.menu_state.game_type as u8))
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                match self
                    .menu_state
                    .selected_level
                    .partial_cmp(&other.menu_state.selected_level)
                {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                self.menu_state
                    .selected_height
                    .partial_cmp(&other.menu_state.selected_height)
            }
        }

        impl<const MODIFIER: Modifier> Ord for Node<MODIFIER> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        let mut past_nodes = BTreeSet::new();
        let mut nodes = BTreeSet::new();
        nodes.insert(Rc::new(Node {
            menu_state: MenuState::<MODIFIER>::new_with_modifier().into(),
            previous_node: None,
            frame: 0,
        }));

        let final_node;
        'search: loop {
            let node = nodes.pop_first().unwrap();

            if past_nodes.contains(&node) {
                continue;
            }
            past_nodes.insert(node.clone());

            for input_byte in (u8::MIN..=u8::MAX).step_by(0x10) {
                let input = Input::from(input_byte);
                let mut new_state = node.as_ref().menu_state.clone();
                let gameplay_state_option = new_state.step(input);
                let new_node = Rc::new(Node {
                    menu_state: new_state,
                    previous_node: Some(Rc::clone(&node)),
                    frame: node.frame + 1,
                });
                match gameplay_state_option {
                    Some(gameplay_state) => {
                        if gameplay_state.level == level
                            && gameplay_state.game_type == game_type
                            && new_node.menu_state.random.index == random_index
                        {
                            final_node = new_node;
                            break 'search;
                        }
                    }
                    None => {
                        nodes.insert(new_node);
                    }
                }
            }
        }

        let mut inputs = Vec::new();
        let mut node_option = Some(final_node);
        loop {
            match node_option {
                Some(node) => {
                    inputs.push(node.menu_state.previous_input);
                    node_option = node.previous_node.as_ref().map(|n| Rc::clone(n));
                }
                None => break,
            }
        }
        inputs.reverse();

        inputs
    }
}

#[cfg(test)]
mod tests {
    use crate::{game_type::GameType, meta_state::MetaState, modifier::Modifier};

    #[test]
    fn optimal_menu_inputs_time() {
        let tests = [
            ((280, 0, GameType::A), 287),
            ((281, 0, GameType::A), 288),
            ((282, 0, GameType::A), 289),
            ((283, 0, GameType::A), 290),
            ((284, 0, GameType::A), 291),
            ((285, 0, GameType::A), 292),
            ((286, 0, GameType::A), 293),
            ((287, 0, GameType::A), 294),
            ((288, 0, GameType::A), 295),
            ((289, 0, GameType::A), 288),
            ((290, 0, GameType::A), 290),
        ];

        for ((random_index, level, game_type), frame_count) in tests {
            assert_eq!(
                MetaState::<{ Modifier::empty() }>::get_menu_inputs(level, game_type, random_index)
                    .len(),
                frame_count,
                "level: {level}, game_type: {game_type:?}), random_index: {random_index}"
            );
        }
    }
}
