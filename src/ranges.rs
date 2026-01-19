//! Unary and Disjoint ranges
//!
//! A [`UnaryRange`] represents a contiguous range of values, defined by `low`
//! and `high` values. Both are inclusive.
//! ```text
//!   |---------------|
//!  low     <=     high
//!
//! ```
//!
//!
//! A [`DisjointRange`] represents a range with gaps
//!
//! ```text
//!  |--------|  |-|  |-------|
//! low     high l h low    high
//! ```
//!
//! The `high` of each (correctly constructed) [`UnaryRange`] in a [`DisjointRange`] is
//! lower than the `low` of the subsequent [`UnaryRange`]. Furthermore, ranges separated
//! by a single [step](`Stepped::STEP`) are combined:
//!
//! ```text
//!   0      4 6      10
//!   |------| |------|  ✅
//!
//!   3      6 4      8       
//!   |------| |------|  ❌
//!
//!   0      4 5     10  ->
//!   |------| |------|
//!   |---------------|
//!   0              10
//!   ```
//!
//!   It's important that `low < high` for each individual [`UnaryRange`].
//!   This is only a problem at construction; [`UnaryRange::new`],
//!   [`DisjointRange::new_single_range`] and [`DisjointRange::from_bounds`]
//!   return `None` if this condition doesn't hold. There are corresponding
//!   `_unchecked` methods if you're willing to fly without a net.

use std::cmp::{max, min};

use crate::traits::{Bounded, Stepped, bounded_max, bounded_min};

/// A single contiguous range of values
///
/// ```text
///   |-------|
///  low  <= high
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UnaryRange<T> {
    low: T,
    high: T,
}

impl<T> UnaryRange<T>
where
    T: Copy + Clone + Bounded + Stepped,
{
    /// Create a new [`UnaryRange`] from `low` and `high` values
    pub fn new(low: T, high: T) -> Option<Self> {
        if low <= high {
            Some(Self { low, high })
        } else {
            None
        }
    }

    /// Create a new [`UnaryRange`] from `low` and `high` values
    ///
    /// `low > high` will result in undesired behavior
    pub fn new_unchecked(low: T, high: T) -> Self {
        Self { low, high }
    }

    /// Test whether a value is contained within the range
    pub fn contains(&self, val: &T) -> bool {
        *val >= self.low && *val <= self.high
    }

    /// The current range without `other`
    ///
    /// This is like subtraction, but returns `Option<Vec<Self>>`.
    /// More specifically, one of three outcomes:
    ///
    ///  `None` (instead of an empty vector) if `other` completely covers `orig`
    ///  (`other.low <= orig.low && other.high >= orig.high`)
    ///
    ///  ```text
    ///    orig          orig
    ///    low           high
    ///     |-------------|
    ///   |----------------|
    ///  other           other
    ///   low             high
    ///  ```
    ///
    ///  A one-range vector if `other` covers only part of `orig`
    ///  (`other.high >= orig.low || other.low <= orig.high`)
    ///
    ///  ```text
    ///       orig                 orig
    ///       low                  high
    ///        |--------------------|
    ///    |-------|
    ///  other   other
    ///   low    high
    ///
    ///             results in
    ///
    ///            |----------------|
    ///           new             old
    ///           low             high
    ///         (other
    ///          high)
    ///
    /// ```
    ///
    /// ```text
    ///
    ///       orig                 orig
    ///       low                  high
    ///        |--------------------|
    ///                       |------|
    ///                     other   other
    ///                      low     high
    ///
    ///             results in
    ///
    ///       |--------------|
    ///      orig           new
    ///      low           high
    ///                    (other
    ///                     low)
    /// ```
    ///
    /// A two-range vector if `other` covers the middle of `orig`
    /// (`other.low > orig.low && other.high < orig.high`)
    ///
    ///  ```text
    ///    orig                orig
    ///    low                 high
    ///     |-------------------|
    ///          |-----|
    ///         other other
    ///         low   high
    ///
    ///       results in
    ///
    ///     |---|      |--------|
    ///   orig new   new      orig
    ///   low high   low      high
    ///      (other (other
    ///       low)   high)
    /// ```
    pub fn without(self, other: Self) -> Option<Vec<Self>> {
        if other.high >= self.high {
            if other.low.decrement() > self.low {
                Some(vec![Self::new_unchecked(self.low, other.low.decrement())])
            } else {
                None
            }
        } else if other.low <= self.low {
            if other.high.increment() < self.high {
                Some(vec![Self::new_unchecked(other.high.increment(), self.high)])
            } else {
                None
            }
        } else {
            Some(vec![
                UnaryRange::new_unchecked(self.low, other.low.decrement()),
                UnaryRange::new_unchecked(other.high.increment(), self.high),
            ])
        }
    }
}

impl<T> UnaryRange<T>
where
    T: Ord + Copy + Clone + Bounded + Stepped,
{
    fn complement_ranges(self) -> Vec<UnaryRange<T>> {
        if self.low == bounded_min() && self.high == bounded_max() {
            Vec::default()
        } else if self.low == bounded_min() {
            vec![Self {
                low: self.high.increment(),
                high: bounded_max(),
            }]
        } else if self.high == bounded_max() {
            vec![Self {
                low: bounded_min(),
                high: self.low.decrement(),
            }]
        } else {
            vec![
                Self {
                    low: bounded_min(),
                    high: self.low.decrement(),
                },
                Self {
                    low: self.high.increment(),
                    high: bounded_max(),
                },
            ]
        }
    }

    /// The complement (or "inverse") of this range
    ///
    /// This is equivalent to
    /// ```ignore
    /// UnaryRange { low: bounded_min(), high: bounded_max() }.without(self)
    /// ```
    /// and similarly returns either `None`, a 1-range vector, or a 2-range vector.
    ///
    /// N.B.: it'll return a 2-range vector unless `self.low == bounded_min()` or
    /// `self.high == bounded_max()`.
    pub fn complement(self) -> Option<DisjointRange<T>> {
        let res = DisjointRange::from_ranges(self.complement_ranges());
        if res.ranges.len() > 0 {
            Some(res)
        } else {
            None
        }
    }
}

/// A range with gaps
///
/// ```text
///  |--------|  |-|  |-------|
/// low     high l h low    high
/// ```
#[derive(Clone, Debug)]
pub struct DisjointRange<T> {
    ranges: Vec<UnaryRange<T>>,
}

impl<T> DisjointRange<T>
where
    T: Copy + Clone + Ord + Bounded + Stepped,
{
    /// Create a new (contiguous) range with a single `low` and
    /// `high` value
    pub fn new_single_range(low: T, high: T) -> Option<Self> {
        UnaryRange::new(low, high).map(|r| Self { ranges: vec![r] })
    }

    /// Create a new (contiguous) range with a single `low` and
    /// `high` value
    ///
    /// `low` > `high` will result in undesired behavior
    pub fn new_single_range_unchecked(low: T, high: T) -> Self {
        Self {
            ranges: vec![UnaryRange::new_unchecked(low, high)],
        }
    }

    /// Create a new range from a vector of [`UnaryRange`]s
    pub fn from_ranges(ranges: Vec<UnaryRange<T>>) -> Self {
        Self { ranges }
    }

    /// Create a new range from a series of `(low, high)` pairs
    ///
    /// If any `(low, high)` pair has `low > high`, undesired behavior will result
    pub fn from_bounds_unchecked<I: IntoIterator<Item = (T, T)>>(bounds: I) -> Self {
        Self {
            ranges: bounds
                .into_iter()
                .map(|(low, high)| UnaryRange { low, high })
                .collect(),
        }
    }

    /// Create a new range from a series of `(low, high)` pairs
    pub fn from_bounds<I: IntoIterator<Item = (T, T)>>(bounds: I) -> Option<Self> {
        bounds
            .into_iter()
            .map(|(low, high)| UnaryRange::new(low, high))
            .collect::<Option<Vec<UnaryRange<T>>>>()
            .map(Self::from_ranges)
    }

    /// Create an empty range
    pub fn empty() -> Self {
        Self { ranges: Vec::new() }
    }

    /// Create a range that covers all values
    pub fn entire() -> Self {
        Self::new_single_range_unchecked(bounded_min(), bounded_max())
    }

    /// Test whether the range contains `val`
    pub fn contains(&self, val: T) -> bool {
        for range in self.ranges.iter() {
            if range.contains(&val) {
                return true;
            }
        }
        false
    }

    /// Combine this `DisjointRange` with another, making sure the result
    /// is ordered and merged
    pub fn add_disjoint_range(self, other: DisjointRange<T>) -> Self {
        let mut out = self.ranges;
        out.extend(other.ranges);
        DisjointRange::sort_ranges(&mut out);
        DisjointRange::meld_ranges(&mut out);
        Self { ranges: out }
    }

    /// Add a [`UnaryRange`] to this `DisjointRange`, making sure the result
    /// is ordered and merged
    pub fn add_unary_range(self, to_add: UnaryRange<T>) -> Self {
        let l = self.ranges.len();
        let mut out = self.ranges;
        let mut i = 0;
        let mut inserted = false;
        while i < l {
            if to_add.low < out[i].low {
                out.insert(i, to_add);
                inserted = true;
                break;
            }
            i += 1;
        }
        if !inserted {
            out.push(to_add);
        }
        DisjointRange::meld_ranges(&mut out);
        Self { ranges: out }
    }

    /// Remove a [`UnaryRange`]('s worth of values) from this `DisjointRange`
    pub fn subtract_unary_range(self, to_remove: UnaryRange<T>) -> Self {
        let orig_len = self.ranges.len();
        let mut out = self.ranges;
        let mut i = 0;
        while i < orig_len {
            if out[i].low > to_remove.high {
                break;
            } else if out[i].low <= to_remove.high || out[1].high >= to_remove.low {
                let target = out.remove(i);
                if let Some(new_ranges) = target.without(to_remove) {
                    let insert_len = new_ranges.len();
                    for new_range in new_ranges.into_iter().rev() {
                        out.insert(i, new_range);
                    }
                    if insert_len == 2 {
                        break; // to_remove entirely contained w/in out[i], we can stop
                    }
                }
            }
            i += 1;
        }
        Self { ranges: out }
    }

    /// The complement (or "inverse") of this range
    ///
    /// This is the combination of the complement of the [`UnaryRange`]s this
    /// `DisjointRange` contains
    pub fn complement(self) -> Self {
        let mut out = self
            .ranges
            .into_iter()
            .flat_map(UnaryRange::complement_ranges)
            .collect();
        DisjointRange::sort_ranges(&mut out);
        DisjointRange::meld_ranges(&mut out);
        Self { ranges: out }
    }

    fn sort_ranges(ranges: &mut Vec<UnaryRange<T>>) {
        ranges.sort_by_cached_key(|UnaryRange { low, .. }: &UnaryRange<T>| *low);
    }

    fn meld_ranges(ranges: &mut Vec<UnaryRange<T>>) {
        let mut i = 0;
        let mut l = ranges.len();
        while i + 1 < l {
            if ranges[i + 1].low <= ranges[i].high.increment() {
                ranges[i + 1].low = min(ranges[i].low, ranges[i + 1].low);
                ranges[i + 1].high = max(ranges[i].high, ranges[i + 1].high);
                ranges.remove(i);
                l -= 1;
            } else {
                i += 1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{DisjointRange, UnaryRange};
    #[test]
    fn test_without_lower() {
        let range = UnaryRange::new_unchecked(5, 10);
        let wo_range = UnaryRange::new_unchecked(3, 6);
        let actual = range.without(wo_range).unwrap();
        assert_eq!(1, actual.len());
        assert_eq!(7, actual[0].low);
        assert_eq!(10, actual[0].high);
    }
    #[test]
    fn test_without_higher() {
        let range = UnaryRange::new_unchecked(5, 10);
        let wo_range = UnaryRange::new_unchecked(8, 12);
        let actual = range.without(wo_range).unwrap();
        assert_eq!(1, actual.len());
        assert_eq!(5, actual[0].low);
        assert_eq!(7, actual[0].high);
    }
    #[test]
    fn test_without_middle() {
        let range = UnaryRange::new_unchecked(5, 10);
        let wo_range = UnaryRange::new_unchecked(7, 8);
        let actual = range.without(wo_range).unwrap();
        assert_eq!(2, actual.len());
        let actual_smaller = actual[0];
        let actual_greater = actual[1];
        assert_eq!(5, actual_smaller.low);
        assert_eq!(6, actual_smaller.high);
        assert_eq!(9, actual_greater.low);
        assert_eq!(10, actual_greater.high);
    }
    #[test]
    fn test_meld_ranges_overlapping() {
        let mut ranges = vec![
            UnaryRange::new_unchecked(0, 9),
            UnaryRange::new_unchecked(10, 19),
        ];
        DisjointRange::meld_ranges(&mut ranges);
        assert_eq!(1, ranges.len());
        let UnaryRange { low, high } = ranges[0];
        assert_eq!(0, low);
        assert_eq!(19, high);
    }

    #[test]
    fn test_meld_ranges_non_overlapping() {
        let mut ranges = vec![
            UnaryRange::new_unchecked(0, 9),
            UnaryRange::new_unchecked(11, 19),
        ];
        DisjointRange::meld_ranges(&mut ranges);
        assert_eq!(2, ranges.len());
        let UnaryRange { low, high } = ranges[0];
        assert_eq!(0, low);
        assert_eq!(9, high);
        let UnaryRange { low, high } = ranges[1];
        assert_eq!(11, low);
        assert_eq!(19, high);
    }
    #[test]
    fn test_meld_ranges_first_two() {
        let mut ranges = vec![
            UnaryRange::new_unchecked(0, 4),
            UnaryRange::new_unchecked(5, 9),
            UnaryRange::new_unchecked(11, 15),
        ];
        DisjointRange::meld_ranges(&mut ranges);
        assert_eq!(2, ranges.len());
        let UnaryRange { low, high } = ranges[0];
        assert_eq!(0, low);
        assert_eq!(9, high);
        let UnaryRange { low, high } = ranges[1];
        assert_eq!(11, low);
        assert_eq!(15, high);
    }
    #[test]
    fn test_meld_ranges_last_two() {
        let mut ranges = vec![
            UnaryRange::new_unchecked(0, 4),
            UnaryRange::new_unchecked(6, 10),
            UnaryRange::new_unchecked(11, 15),
        ];
        DisjointRange::meld_ranges(&mut ranges);
        assert_eq!(2, ranges.len());
        let UnaryRange { low, high } = ranges[0];
        assert_eq!(0, low);
        assert_eq!(4, high);
        let UnaryRange { low, high } = ranges[1];
        assert_eq!(6, low);
        assert_eq!(15, high);
    }
    #[test]
    fn test_meld_ranges_all() {
        let mut ranges = vec![
            UnaryRange::new_unchecked(0, 4),
            UnaryRange::new_unchecked(5, 10),
            UnaryRange::new_unchecked(11, 15),
        ];
        DisjointRange::meld_ranges(&mut ranges);
        assert_eq!(1, ranges.len());
        let UnaryRange { low, high } = ranges[0];
        assert_eq!(0, low);
        assert_eq!(15, high);
    }
    #[test]
    fn test_subtract_range_lowest_lower() {
        let orig = DisjointRange::from_bounds_unchecked(vec![(0, 4), (6, 10), (12, 16)]);
        let o2 = orig.clone();
        let wo_range = UnaryRange::new_unchecked(0, 2);
        let subtracted = orig.subtract_unary_range(wo_range);
        assert_eq!(3, subtracted.ranges.len());
        assert_eq!(UnaryRange { low: 3, high: 4 }, subtracted.ranges[0]);
        assert_eq!(o2.ranges[1], subtracted.ranges[1]);
        assert_eq!(o2.ranges[2], subtracted.ranges[2]);
    }
    #[test]
    fn test_subtract_unary_range_lowest_higher() {
        let orig = DisjointRange::from_bounds_unchecked(vec![(0, 4), (6, 10), (12, 16)]);
        let o2 = orig.clone();
        let wo_range = UnaryRange::new_unchecked(2, 4);
        let subtracted = orig.subtract_unary_range(wo_range);
        assert_eq!(3, subtracted.ranges.len());
        assert_eq!(UnaryRange { low: 0, high: 1 }, subtracted.ranges[0]);
        assert_eq!(o2.ranges[1], subtracted.ranges[1]);
        assert_eq!(o2.ranges[2], subtracted.ranges[2]);
    }
    #[test]
    fn test_subtract_unary_range_lowest_middle() {
        let orig = DisjointRange::from_bounds_unchecked(vec![(0, 4), (6, 10), (12, 16)]);
        let o2 = orig.clone();
        let wo_range = UnaryRange::new_unchecked(2, 3);
        let subtracted = orig.subtract_unary_range(wo_range);
        assert_eq!(4, subtracted.ranges.len());
        assert_eq!(UnaryRange { low: 0, high: 1 }, subtracted.ranges[0]);
        assert_eq!(UnaryRange { low: 4, high: 4 }, subtracted.ranges[1]);
        assert_eq!(o2.ranges[1], subtracted.ranges[2]);
        assert_eq!(o2.ranges[2], subtracted.ranges[3]);
    }
    #[test]
    fn test_subtract_unary_range_lower_spanning() {
        let orig = DisjointRange::from_bounds_unchecked(vec![(0, 4), (6, 10), (12, 16)]);
        let o2 = orig.clone();
        let wo_range = UnaryRange::new_unchecked(4, 7);
        let subtracted = orig.subtract_unary_range(wo_range);
        assert_eq!(3, subtracted.ranges.len());
        assert_eq!(UnaryRange { low: 0, high: 3 }, subtracted.ranges[0]);
        assert_eq!(UnaryRange { low: 8, high: 10 }, subtracted.ranges[1]);
        assert_eq!(o2.ranges[2], subtracted.ranges[2]);
    }
    #[test]
    fn test_add_unary_range_before_separate() {
        let orig = DisjointRange::from_bounds_unchecked(vec![(4, 6), (8, 10)]);
        let o2 = orig.clone();
        let to_add = UnaryRange::new_unchecked(0, 2);
        let ta = to_add.clone();
        let added = orig.add_unary_range(to_add);
        assert_eq!(3, added.ranges.len());
        assert_eq!(ta, added.ranges[0]);
        assert_eq!(o2.ranges[0], added.ranges[1]);
        assert_eq!(o2.ranges[1], added.ranges[2]);
    }
    #[test]
    fn test_add_unary_range_before_merged() {
        let orig = DisjointRange::from_bounds_unchecked(vec![(4, 6), (8, 10)]);
        let o2 = orig.clone();
        let to_add = UnaryRange::new_unchecked(0, 3);
        let added = orig.add_unary_range(to_add);
        assert_eq!(2, added.ranges.len());
        assert_eq!(UnaryRange { low: 0, high: 6 }, added.ranges[0]);
        assert_eq!(o2.ranges[1], added.ranges[1]);
    }
    #[test]
    fn test_add_unary_range_middle_separate() {
        let orig = DisjointRange::from_bounds_unchecked(vec![(4, 5), (10, 11)]);
        let o2 = orig.clone();
        let to_add = UnaryRange::new_unchecked(7, 8);
        let ta = to_add.clone();
        let added = orig.add_unary_range(to_add);
        assert_eq!(3, added.ranges.len());
        assert_eq!(o2.ranges[0], added.ranges[0]);
        assert_eq!(ta, added.ranges[1]);
        assert_eq!(o2.ranges[1], added.ranges[2]);
    }
    #[test]
    fn test_add_unary_range_middle_merged() {
        let orig = DisjointRange::from_bounds_unchecked(vec![(4, 5), (10, 11)]);
        let o2 = orig.clone();
        let to_add = UnaryRange::new_unchecked(6, 8);
        let added = orig.add_unary_range(to_add);
        assert_eq!(2, added.ranges.len());
        assert_eq!(UnaryRange { low: 4, high: 8 }, added.ranges[0]);
        assert_eq!(o2.ranges[1], added.ranges[1]);
    }
    #[test]
    fn test_add_unary_range_merge_all() {
        let orig = DisjointRange::from_bounds_unchecked(vec![(4, 5), (10, 11)]);
        let to_add = UnaryRange::new_unchecked(6, 9);
        let added = orig.add_unary_range(to_add);
        assert_eq!(1, added.ranges.len());
        assert_eq!(UnaryRange { low: 4, high: 11 }, added.ranges[0]);
    }
}
