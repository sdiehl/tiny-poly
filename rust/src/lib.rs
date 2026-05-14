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
