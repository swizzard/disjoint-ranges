//! Relevant traits
//!
//! [`Bounded`] and [`Stepped`] are already [implemented for standard numeric types](crate::impls).
//! You're obviously welcome to implement these traits for your own custom types, but given that
//! [`UnaryRange`](crate::ranges::UnaryRange) and [`DisjointRange`](crate::ranges::DisjointRange)
//! both require `T: Copy + Clone + Bounded + Stepped` and `DisjointRange` additionally requires
//! `T: Ord`, the space of available (distinct, meaningful) types is a bit limited. Maybe tuples?

/// Trait for types with minimum and maximum values
///
/// Obviously, it should be true that `T::MIN_VAL <= T::MAX_VAL` for any `T: impl Bounded`
pub trait Bounded: PartialOrd {
    const MIN_VAL: Self;
    const MAX_VAL: Self;
}

/// Trait for types whose values are (partially) ordered and separated by
/// a consistent, measurable quantity
///
/// For any `v: impl Stepped`:
/// `v.decrement() <= v <= v.increment()`
/// `v.increment().decrement() == v.decrement().increment() == v`
pub trait Stepped: Bounded {
    const STEP: Self;
    /// Increase by [`Stepped::STEP`]
    fn increment(&self) -> Self;

    /// Decrease by [`Stepped::STEP`]
    fn decrement(&self) -> Self;
}

/// Helper function providing a type's [`Bounded::MIN_VAL`]
pub fn bounded_min<T: Bounded>() -> T {
    T::MIN_VAL
}

/// Helper function providing a type's [`Bounded::MAX_VAL`]
pub fn bounded_max<T: Bounded>() -> T {
    T::MAX_VAL
}
