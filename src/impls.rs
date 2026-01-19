//! [Bounded] and [Stepped] implementations for standard numeric types.
//!
//! Since `f32` and `f64` only implement `PartialOrd`, they only be used to construct
//! [UnaryRange](crate::ranges::UnaryRange)s

use crate::traits::{Bounded, Stepped};
use std::cmp::{max, min};

impl Stepped for u8 {
    const STEP: u8 = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for u8 {
    const MIN_VAL: u8 = std::u8::MIN;
    const MAX_VAL: u8 = std::u8::MAX;
}

impl Stepped for u16 {
    const STEP: u16 = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for u16 {
    const MIN_VAL: u16 = std::u16::MIN;
    const MAX_VAL: u16 = std::u16::MAX;
}
impl Stepped for u32 {
    const STEP: u32 = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for u32 {
    const MIN_VAL: u32 = std::u32::MIN;
    const MAX_VAL: u32 = std::u32::MAX;
}
impl Stepped for u64 {
    const STEP: u64 = 1;

    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for u64 {
    const MIN_VAL: u64 = std::u64::MIN;
    const MAX_VAL: u64 = std::u64::MAX;
}
impl Stepped for u128 {
    const STEP: u128 = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for u128 {
    const MIN_VAL: u128 = std::u128::MIN;
    const MAX_VAL: u128 = std::u128::MAX;
}
impl Stepped for usize {
    const STEP: usize = 1;

    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for usize {
    const MIN_VAL: usize = std::usize::MIN;
    const MAX_VAL: usize = std::usize::MAX;
}
impl Stepped for i8 {
    const STEP: i8 = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for i8 {
    const MIN_VAL: i8 = std::i8::MIN;
    const MAX_VAL: i8 = std::i8::MAX;
}

impl Stepped for i16 {
    const STEP: i16 = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for i16 {
    const MIN_VAL: i16 = std::i16::MIN;
    const MAX_VAL: i16 = std::i16::MAX;
}
impl Stepped for i32 {
    const STEP: i32 = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for i32 {
    const MIN_VAL: i32 = std::i32::MIN;
    const MAX_VAL: i32 = std::i32::MAX;
}
impl Stepped for i64 {
    const STEP: i64 = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for i64 {
    const MIN_VAL: i64 = std::i64::MIN;
    const MAX_VAL: i64 = std::i64::MAX;
}
impl Stepped for i128 {
    const STEP: i128 = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for i128 {
    const MIN_VAL: i128 = std::i128::MIN;
    const MAX_VAL: i128 = std::i128::MAX;
}
impl Stepped for isize {
    const STEP: isize = 1;
    fn increment(&self) -> Self {
        self.saturating_add(Self::STEP)
    }
    fn decrement(&self) -> Self {
        self.saturating_sub(Self::STEP)
    }
}

impl Bounded for isize {
    const MIN_VAL: isize = std::isize::MIN;
    const MAX_VAL: isize = std::isize::MAX;
}

impl Stepped for f32 {
    const STEP: f32 = f32::EPSILON;
    fn increment(&self) -> Self {
        self + Self::STEP
    }
    fn decrement(&self) -> Self {
        self - Self::STEP
    }
}

impl Bounded for f32 {
    const MIN_VAL: f32 = std::f32::NEG_INFINITY;
    const MAX_VAL: f32 = std::f32::INFINITY;
}
impl Stepped for f64 {
    const STEP: f64 = std::f64::INFINITY;

    fn increment(&self) -> Self {
        self + Self::STEP
    }
    fn decrement(&self) -> Self {
        self - Self::STEP
    }
}

impl Bounded for f64 {
    const MIN_VAL: f64 = std::f64::NEG_INFINITY;
    const MAX_VAL: f64 = std::f64::INFINITY;
}

impl Bounded for char {
    const MIN_VAL: char = char::MIN;
    const MAX_VAL: char = char::MAX;
}

impl Stepped for char {
    const STEP: char = 1 as char;
    fn increment(&self) -> Self {
        char::from_u32(min((*self as u32).saturating_add(1), char::MAX as u32)).unwrap()
    }
    fn decrement(&self) -> Self {
        char::from_u32(max((*self as u32).saturating_sub(1), char::MIN as u32)).unwrap()
    }
}
