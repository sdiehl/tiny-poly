//! Polynomial functors as a category: positions, directions, lenses, Moore machines, comonoids.

#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::many_single_char_names,
    clippy::similar_names,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

pub mod comonoid;
pub mod lens;
pub mod moore;
pub mod ops;
pub mod poly;
pub mod wiring;

pub use comonoid::{Category, Comonoid, Morphism};
pub use lens::Lens;
pub use moore::Moore;
pub use ops::{compose, sum, tensor};
pub use poly::{Poly, Position};
pub use wiring::{parallel, series};
