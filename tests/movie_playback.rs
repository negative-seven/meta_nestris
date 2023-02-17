#![allow(incomplete_features)]
#![feature(adt_const_params)]

use meta_nestris::Modifier;
use meta_nestris::{Input, MenuMode, Movie, State};
use serde::Deserialize;
use serde::Deserializer;
use std::{collections::HashMap, fs::File, iter::repeat, path::PathBuf};

#[derive(Deserialize)]
struct MovieData {
    filename: String,
    #[serde(default)] // default to false
    uncapped_score: bool,
    checks: HashMap<u32, MovieCheck>,
}

#[derive(Deserialize)]
struct MovieCheck {
    score: Option<u32>,
    line_count: Option<u16>,
    dead: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_menu_mode")]
    menu_mode: Option<MenuMode>,
    is_gameplay_state: Option<bool>,
}

pub fn deserialize_menu_mode<'de, D>(deserializer: D) -> Result<Option<MenuMode>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(remote = "MenuMode")]
    enum MenuModeDuplicate {
        CopyrightScreen,
        TitleScreen,
        GameTypeSelect,
        LevelSelect,
        InitializingGame,
    }

    #[derive(Deserialize)]
    struct MenuModeWrapper(#[serde(with = "MenuModeDuplicate")] MenuMode);

    Ok(Option::deserialize(deserializer)?.map(|MenuModeWrapper(menu_mode)| menu_mode))
}

#[test]
fn movie_playback() {
    let metadata_json: Vec<MovieData> =
        serde_yaml::from_reader(File::open("tests/movies/metadata.yaml").unwrap()).unwrap();

    for movie_data in metadata_json {
        let movie_full_filepath = PathBuf::from("tests/movies/").join(movie_data.filename);
        let movie = Movie::from_fm2(&movie_full_filepath).unwrap_or_else(|_| {
            panic!(
                "could not open movie file: {}",
                movie_full_filepath.display()
            )
        });
        let inputs = movie.inputs.into_iter().chain(repeat(Input::None));

        if movie_data.uncapped_score {
            const MODIFIER: Modifier = Modifier {
                uncapped_score: true,
                ..Modifier::empty()
            };
            check_movie::<MODIFIER>(&movie_data.checks, inputs);
        } else {
            check_movie::<{ Modifier::empty() }>(&movie_data.checks, inputs);
        }
    }
}

fn check_movie<const MODIFIER: Modifier>(
    checks: &HashMap<u32, MovieCheck>,
    inputs: impl Iterator<Item = Input>,
) {
    // may need to play movie beyond final stored input
    // at the same time, do not need to play movie beyond last checked frame
    let playback_duration = *checks.keys().max().unwrap();

    let mut state = State::<MODIFIER>::new_with_modifier();
    for (input, frame) in inputs.zip(1..=playback_duration) {
        state.step(input);

        if let Some(check) = checks.get(&frame) {
            check_state(&state, check);
        }
    }
}

fn check_state<const MODIFIER: Modifier>(state: &State<MODIFIER>, check: &MovieCheck) {
    if let Some(score) = check.score {
        match state {
            State::MenuState(_) => panic!("found menu state during score check"),
            State::GameplayState(state) => assert_eq!(score, state.score),
        }
    }

    if let Some(line_count) = check.line_count {
        match state {
            State::MenuState(_) => panic!("found menu state during line count check"),
            State::GameplayState(state) => assert_eq!(line_count, state.line_count),
        }
    }

    if let Some(dead) = check.dead {
        match state {
            State::MenuState(_) => panic!("found menu state during death check"),
            State::GameplayState(state) => assert_eq!(dead, state.dead),
        }
    }

    if let Some(menu_mode) = check.menu_mode {
        match state {
            State::MenuState(state) => assert_eq!(menu_mode, state.menu_mode),
            State::GameplayState(_) => panic!("found gameplay state during menu mode check"),
        }
    }

    if let Some(is_gameplay_state) = check.is_gameplay_state {
        if is_gameplay_state {
            if let State::MenuState(_) = state {
                panic!("found menu state when expecting gameplay state");
            }
        } else if let State::GameplayState(_) = state {
            panic!("found gameplay state when expecting menu state")
        }
    }
}
