//! A largely functionally accurate recreation of NTSC NES Tetris, with many
//! non-essential redundancies such as audio and graphics related logic stripped
//! away.
//!
//! This crate provides the following types to represent states of the game:
//! * [`State`] represents any state of the game.
//! * [`GameplayState`] represents a de facto gameplay state of the game; i.e. a
//!   state where the playfield is present.
//! * [`MenuState`] represents all states not covered by `GameplayState`,
//!   referred to as menu screens.
//!
//! States correspond to instances between frames of the game, or the instance
//! before the first frame of the game. Boundaries between frames loosely
//! (though not exactly) correspond to the NES' NMI occurrances. This
//! correspondence is analogous to how the NES is typically emulated, and as
//! such, this crate is fit to play back emulator input files (also known as
//! movies). The [`Movie`] struct can aid in emulator movie
//! playback:
//! ```no_run
//! use meta_nestris::{Movie, State};
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
//! * Some operations have been intentionally altered in ways that simplify
//!   logic, but do not ultimately affect accuracy. An example of this is the
//!   `shift_autorepeat` field in `GameplayState` counting down, rather than up
//!   like its respective variable in the original game.
//!
//! Further modifications to the game, such as preventing the score from being
//! capped at 999999, can also be applied: see the [`Modifier`] type for
//! details.

#![allow(incomplete_features)]
#![feature(adt_const_params)]

mod game_mode_state;
mod game_type;
mod gameplay_state;
mod input;
mod menu_mode;
mod menu_state;
mod modifier;
mod movie;
mod piece;
mod play_state;
mod random;
mod state;

pub use game_mode_state::*;
pub use game_type::*;
pub use gameplay_state::*;
pub use input::*;
pub use menu_mode::*;
pub use menu_state::*;
pub use modifier::*;
pub use movie::*;
pub use piece::*;
pub use play_state::*;
pub use random::*;
pub use state::*;
