use meta_nestris::{game_type::GameType, meta_state::MetaState, state::State};

#[test]
fn meta_state_equivalent() {
    let meta_state = MetaState::new(0, GameType::A);
    let inputs = meta_state.get_inputs();
    assert_eq!(inputs.len(), meta_state.frame.try_into().unwrap());

    let gameplay_state_from_meta_state = meta_state.to_gameplay_state();

    let mut state_from_inputs = State::new();
    for input in inputs {
        state_from_inputs.step(input);
    }
    let gameplay_state_from_inputs = match state_from_inputs {
        State::MenuState(_) => panic!("menu state reached after inputs"),
        State::GameplayState(state) => state,
    };

    assert_eq!(gameplay_state_from_meta_state, gameplay_state_from_inputs);
}
