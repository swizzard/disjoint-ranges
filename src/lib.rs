//! Unary and Disjoint ranges plus some useful traits

/// Trait implementations for standard numeric types
pub mod impls;
/// Contiguous and disjoint ranges
pub mod ranges;
/// Helpful traits
pub mod traits;

pub use ranges::{DisjointRange, UnaryRange};
pub use traits::{Bounded, Stepped};
