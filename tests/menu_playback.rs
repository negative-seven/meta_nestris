use meta_nestris::{movie::Movie, state::State};
use std::{fs::read_dir, path::PathBuf};

fn get_last_state(movie_filepath: &PathBuf) -> State {
    let movie = Movie::from_fm2(&movie_filepath.into()).unwrap();

    let mut state = State::new();
    for input in movie.inputs {
        state.step(&input);
    }
    return state;
}

fn reach_game_mode(directory: &str, target_game_mode: u8) {
    let mut found_any_movie = false;

    for filepath in read_dir(directory).unwrap().map(|p| p.unwrap().path()) {
        found_any_movie = true;

        let state = get_last_state(&filepath);
        assert!(
            state.game_mode == target_game_mode,
            "{}: game mode = {}",
            filepath.display(),
            state.game_mode
        );
    }

    if !found_any_movie {
        panic!("no movies found")
    }
}

#[test]
fn reach_copyright() {
    reach_game_mode(r"tests\movies\menus\copyright", 0);
}

#[test]
fn reach_title() {
    reach_game_mode(r"tests\movies\menus\title", 1);
}

#[test]
fn reach_game_type() {
    reach_game_mode(r"tests\movies\menus\game_type", 2);
}

#[test]
fn reach_level_select() {
    reach_game_mode(r"tests\movies\menus\level_select", 3);
}

#[test]
fn reach_gameplay() {
    reach_game_mode(r"tests\movies\menus\gameplay", 4);
}
