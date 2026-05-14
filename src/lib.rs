//! Polynomial functors as a category, with sum, parallel tensor, and composition

/// Greet by name.
#[must_use]
pub fn greet(name: &str) -> String {
    format!("hello, {name}")
}
