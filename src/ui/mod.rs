//! Widgets

pub mod app;
mod errors;
pub(crate) mod selection_listbox;
pub(crate) mod number_input;
pub(crate) mod profile_chooser;

pub use app::App;
use errors::Errors;
use number_input::NumberInput;
use profile_chooser::ProfileChooser;