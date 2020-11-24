//! A crate providing access to the NanoWrimo API, in its public and private forms.
//!
//! Currently, there is no public API. As such, this crate may break at any time. Please
//! direct any issues to [The Issue Tracker](https://github.com/CraftSpider/nanowrimo-rs)

mod utils;
mod kind;
mod enums;

pub mod data;
pub mod error;
pub mod client;

pub use kind::NanoKind;
pub use enums::*;
pub use data::*;
pub use error::Error;
pub use client::NanoClient;
