use meta_nestris::{movie::Movie, state::State};
use std::{fs::read_dir, path::PathBuf};

fn get_last_two_states(movie_filepath: &PathBuf) -> (State, State) {
    let movie = Movie::from_fm2(movie_filepath).unwrap();

    let mut state = State::new();
    for input in movie.inputs.iter().take(movie.inputs.len() - 1) {
        state.step(*input);
    }

    let next_to_last_state = state.clone();
    state.step(*movie.inputs.last().unwrap());

    (next_to_last_state, state)
}

#[test]
fn a_type_max_score_playback() {
    let mut found_any_movie = false;

    for filepath in read_dir(r"tests\movies\a_type_max_score")
        .unwrap()
        .map(|p| p.unwrap().path())
    {
        found_any_movie = true;

        let (next_to_last_state, last_state) = get_last_two_states(&filepath);

        match next_to_last_state {
            State::MenuState(_) => panic!("next to last state is a menu state"),
            State::GameplayState(state) => assert!(
                state.score != 999999,
                "{}: maximum score reached before last state",
                filepath.display()
            ),
        }

        match last_state {
            State::MenuState(_) => panic!("last state is a menu state"),
            State::GameplayState(state) => {
                assert!(
                    state.score == 999999,
                    "{}: maximum score not reached in last state",
                    filepath.display()
                );
            }
        }
    }

    if !found_any_movie {
        panic!("no movies found")
    }
}

#[test]
fn b_type_clear_playback() {
    let mut found_any_movie = false;

    for filepath in read_dir(r"tests\movies\b_type_clear")
        .unwrap()
        .map(|p| p.unwrap().path())
    {
        found_any_movie = true;

        let (next_to_last_state, last_state) = get_last_two_states(&filepath);

        match next_to_last_state {
            State::MenuState(_) => panic!("next to last state is a menu state"),
            State::GameplayState(state) => assert!(
                state.lines != 0,
                "{}: 0 lines reached before last state",
                filepath.display()
            ),
        }

        match last_state {
            State::MenuState(_) => panic!("last state is a menu state"),
            State::GameplayState(state) => assert!(
                state.lines == 0,
                "{}: 0 lines not reached in last state",
                filepath.display()
            ),
        };
    }

    if !found_any_movie {
        panic!("no movies found")
    }
}

#[test]
fn death_playback() {
    let mut found_any_movie = false;

    for filepath in read_dir(r"tests\movies\death")
        .unwrap()
        .map(|p| p.unwrap().path())
    {
        found_any_movie = true;

        let (next_to_last_state, last_state) = get_last_two_states(&filepath);

        match next_to_last_state {
            State::MenuState(_) => panic!("next to last state is a menu state"),
            State::GameplayState(state) => assert!(
                !state.dead,
                "{}: dead before last state",
                filepath.display()
            ),
        }

        match last_state {
            State::MenuState(_) => panic!("last state is a menu state"),
            State::GameplayState(state) => {
                assert!(state.dead, "{}: not dead in last state", filepath.display())
            }
        }
    }

    if !found_any_movie {
        panic!("no movies found")
    }
}
