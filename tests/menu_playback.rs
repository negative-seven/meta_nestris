use meta_nestris::{menu_mode::MenuMode, movie::Movie, state::State};
use std::{fs::read_dir, path::PathBuf};

fn get_last_state(movie_filepath: &PathBuf) -> State {
    let movie = Movie::from_fm2(&movie_filepath.into()).unwrap();

    let mut state = State::new();
    for input in movie.inputs {
        state.step(input);
    }
    return state;
}

fn reach_game_mode(directory: &str, target_game_mode: MenuMode) {
    let mut found_any_movie = false;

    for filepath in read_dir(directory).unwrap().map(|p| p.unwrap().path()) {
        found_any_movie = true;

        let state = get_last_state(&filepath);
        match state {
            State::MenuState(state) => {
                assert!(
                    state.game_mode == target_game_mode,
                    "{}: game mode = {}",
                    filepath.display(),
                    state.game_mode
                );
            }
            State::GameplayState(_) => {
                panic!("last state is a gameplay state");
            }
        }
    }

    if !found_any_movie {
        panic!("no movies found")
    }
}

#[test]
fn reach_copyright() {
    reach_game_mode(r"tests\movies\menus\copyright", MenuMode::CopyrightScreen);
}

#[test]
fn reach_title() {
    reach_game_mode(r"tests\movies\menus\title", MenuMode::TitleScreen);
}

#[test]
fn reach_game_type() {
    reach_game_mode(r"tests\movies\menus\game_type", MenuMode::GameTypeSelect);
}

#[test]
fn reach_level_select() {
    reach_game_mode(r"tests\movies\menus\level_select", MenuMode::LevelSelect);
}

#[test]
fn reach_gameplay() {
    let mut found_any_movie = false;

    for filepath in read_dir(r"tests\movies\menus\gameplay")
        .unwrap()
        .map(|p| p.unwrap().path())
    {
        found_any_movie = true;

        let state = get_last_state(&filepath);
        if let State::MenuState(_) = state {
            panic!("last state is a menu state");
        }
    }

    if !found_any_movie {
        panic!("no movies found")
    }
}
