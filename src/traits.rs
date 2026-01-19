//! Relevant traits

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
