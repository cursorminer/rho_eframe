#![warn(clippy::all, rust_2018_idioms)]

use rand;

mod app;
pub use app::TemplateApp;
pub mod clock;
pub mod clock_runner;
pub mod grid_activations;
pub mod gui_runner;
pub mod looping_state;
pub mod messages;
pub mod midi_helpers;
pub mod note_assigner;
pub mod phasor;
pub mod rho;
pub mod rho_config;
pub mod step_switch;
