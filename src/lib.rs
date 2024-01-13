//! Raw curses bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::{addnstr, endwin, getch, initscr, refresh};
pub use types::Window;
