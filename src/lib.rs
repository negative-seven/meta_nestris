//! A largely functionally accurate recreation of NTSC NES Tetris, with many
//! non-essential redundancies such as audio and graphics related logic stripped
//! away.
//!
//! This crate provides the following types to represent states of the game:
//! * [`State`](state::State) represents any state of the game.
//! * [`GameplayState`](gameplay_state::GameplayState) represents a de facto
//!   gameplay state of the game; i.e. a state where the playfield is present.
//! * [`MenuState`][menu_state::MenuState] represents all states not covered by
//!   `GameplayState`, referred to as menu screens.
//!
//! States correspond to instances between frames of the game, or the instance
//! before the first frame of the game. Boundaries between frames loosely
//! (though not exactly) correspond to the NES' NMI occurrances. This
//! correspondence is analogous to how the NES is typically emulated, and as
//! such, this crate is fit to play back emulator input files (also known as
//! movies). The [`Movie`](movie::Movie) struct can aid in emulator movie
//! playback:
//! ```no_run
//! use meta_nestris::{movie::Movie, state::State};
//!
//! let movie = Movie::from_fm2(&"inputs.fm2".into()).expect("File not found.");
//!
//! let mut state = State::new();
//! for input in movie.inputs {
//!     state.step(input);
//! }
//!
//! match state {
//!     State::GameplayState(gameplay_state) => {
//!         println!("gameplay - score: {}", gameplay_state.score);
//!     }
//!     State::MenuState(menu_state) => {
//!         println!("menu: {}", menu_state.menu_mode);
//!     }
//! }
//! ```
//!
//! Although this crate aims for substantive accuracy to the original game,
//! there ineviteably exist differences between the two. The known deviations
//! are:
//! * Console resets and the in-game A+B+select+start button combo to reset the
//!   game are unsupported.
//! * The demo never plays.
//! * Gameplay continues after the B-type goal is reached.
//! * The state remains unchanged once the player loses.
//! * Lag and program counter corruption at high levels is not emulated.
//! * The player's score is not capped at 999999. This is intentional.
//! * Some operations have been intentionally altered in ways that simplify
//!   logic, but do not ultimately affect accuracy. An example of this is the
//!   `shift_autorepeat` field in `GameplayState` counting down, rather than up
//!   like its respective variable in the original game.

pub mod game_mode_state;
pub mod game_type;
pub mod gameplay_state;
pub mod input;
pub mod menu_mode;
pub mod menu_state;
pub mod movie;
pub mod piece;
pub mod play_state;
pub mod random;
pub mod state;
