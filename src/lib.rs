#![no_std]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

/// Enable allocations despite being `no_std`.
#[macro_use]
extern crate alloc;

/// Data structures and convenient methods for working with musical harmonies and chords.
pub mod chord;
/// Error types for this library.
pub mod error;
/// Data structures and convenient methods for working with musical notes and MIDI messages.
pub mod note;

/// Exports all the core features of this library through a simple export.
///
/// ```rust
/// use whatthechord::prelude::*;
///
/// /* Now you have access to `Note`, `Chord`, etc.*/
/// ```
pub mod prelude {
    pub use crate::{chord::qualities::*, chord::*, note::*};
}
