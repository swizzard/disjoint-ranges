//! Unary and Disjoint ranges
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
//! by a single (step)[`Stepped::STEP`] are combined:
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
    T: Clone + Bounded + Stepped,
{
    /// Create a new [`UnaryRange`] from `low` and `high` values
    ///
    /// This is unchecked; `low > high` will result in undesired
    /// behavior.
    pub fn new(min: T, max: T) -> Self {
        Self {
            low: min,
            high: max,
        }
    }
    /// Test whether a value is contained within the range
    pub fn contains(&self, val: &T) -> bool {
        *val >= self.low && *val <= self.high
    }
    /// The current range without `other`.
    ///
    /// This is like subtraction, but returns `Option<Vec<Self>>`.
    /// More specifically, one of three scenarios:
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
                Some(vec![Self::new(self.low, other.low.decrement())])
            } else {
                None
            }
        } else if other.low <= self.low {
            if other.high.increment() < self.high {
                Some(vec![Self::new(other.high.increment(), self.high)])
            } else {
                None
            }
        } else {
            Some(vec![
                UnaryRange::new(self.low, other.low.decrement()),
                UnaryRange::new(other.high.increment(), self.high),
            ])
        }
    }
}

impl<T> UnaryRange<T>
where
    T: Copy + Clone + Ord + Bounded + Stepped,
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
    /// ```
    /// UnaryRange { low: bounded_min(), high: bounded_max() }.without(self)
    /// ```
    /// and similarly returns either `None`, a 1-range vector, or a 2-range vector.
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

#[derive(Clone, Debug)]
pub struct DisjointRange<T> {
    ranges: Vec<UnaryRange<T>>,
}

/// A range with gaps
///
/// ```text
///  |--------|  |-|  |-------|
/// low     high l h low    high
/// ```
impl<T> DisjointRange<T>
where
    T: Copy + Clone + Ord + Bounded + Stepped,
{
    /// Create a new (contiguous) range with a single `low` and
    /// `high` value
    pub fn new_single_range(low: T, high: T) -> Self {
        Self {
            ranges: vec![UnaryRange::new(low, high)],
        }
    }
    /// Create a new range from a vector of [`UnaryRange`]s
    pub fn from_ranges(ranges: Vec<UnaryRange<T>>) -> Self {
        Self { ranges }
    }
    #[cfg(test)]
    fn from_bounds(bounds: Vec<(T, T)>) -> Self {
        Self {
            ranges: bounds
                .into_iter()
                .map(|(low, high)| UnaryRange { low, high })
                .collect(),
        }
    }
    /// Create an empty range
    pub fn empty() -> Self {
        Self { ranges: Vec::new() }
    }
    /// Create a range that covers all values
    pub fn entire() -> Self {
        Self::new_single_range(bounded_min(), bounded_max())
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
    /// Combine this [`DisjointRange`] with another, making sure the result
    /// is ordered and merged
    pub fn add_disjoint_range(self, DisjointRange { ranges: shorter }: DisjointRange<T>) -> Self {
        let mut out = self.ranges;
        out.extend(shorter);
        DisjointRange::sort_ranges(&mut out);
        DisjointRange::meld_ranges(&mut out);
        Self { ranges: out }
    }
    /// Add a [`UnaryRange`] to this [`DisjointRange`], making sure the result
    /// is ordered and merged
    pub fn add_unary_range(self, to_insert: UnaryRange<T>) -> Self {
        let l = self.ranges.len();
        let mut out = self.ranges;
        let mut i = 0;
        while i < l {
            if to_insert.low < out[i].low {
                out.insert(i, to_insert);
                return Self { ranges: out };
            }
            i += 1;
        }
        out.push(to_insert);
        DisjointRange::meld_ranges(&mut out);
        Self { ranges: out }
    }
    /// Remove a [`UnaryRange`]('s worth of values) from this [`DisjointRange`]
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
impl<T> DisjointRange<T>
where
    T: Copy + Clone + Ord + Bounded + Stepped,
{
    /// The complement (or "inverse") of this range
    ///
    /// This is the combination of the complement of the [`UnaryRange`]s this
    /// [`DisjointRange`] contains.
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
}
#[cfg(test)]
mod tests {
    use super::{DisjointRange, UnaryRange};
    #[test]
    fn test_without_lower() {
        let range = UnaryRange::new(5, 10);
        let wo_range = UnaryRange::new(3, 6);
        let actual = range.without(wo_range).unwrap();
        assert_eq!(1, actual.len());
        assert_eq!(7, actual[0].low);
        assert_eq!(10, actual[0].high);
    }
    #[test]
    fn test_without_higher() {
        let range = UnaryRange::new(5, 10);
        let wo_range = UnaryRange::new(8, 12);
        let actual = range.without(wo_range).unwrap();
        assert_eq!(1, actual.len());
        assert_eq!(5, actual[0].low);
        assert_eq!(7, actual[0].high);
    }
    #[test]
    fn test_without_middle() {
        let range = UnaryRange::new(5, 10);
        let wo_range = UnaryRange::new(7, 8);
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
        let mut ranges = vec![UnaryRange::new(0, 9), UnaryRange::new(10, 19)];
        DisjointRange::meld_ranges(&mut ranges);
        assert_eq!(1, ranges.len());
        let UnaryRange { low, high } = ranges[0];
        assert_eq!(0, low);
        assert_eq!(19, high);
    }

    #[test]
    fn test_meld_ranges_non_overlapping() {
        let mut ranges = vec![UnaryRange::new(0, 9), UnaryRange::new(11, 19)];
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
            UnaryRange::new(0, 4),
            UnaryRange::new(5, 9),
            UnaryRange::new(11, 15),
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
            UnaryRange::new(0, 4),
            UnaryRange::new(6, 10),
            UnaryRange::new(11, 15),
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
            UnaryRange::new(0, 4),
            UnaryRange::new(5, 10),
            UnaryRange::new(11, 15),
        ];
        DisjointRange::meld_ranges(&mut ranges);
        assert_eq!(1, ranges.len());
        let UnaryRange { low, high } = ranges[0];
        assert_eq!(0, low);
        assert_eq!(15, high);
    }
    #[test]
    fn test_subtract_range_lowest_lower() {
        let orig = DisjointRange::from_bounds(vec![(0, 4), (6, 10), (12, 16)]);
        let o2 = orig.clone();
        let wo_range = UnaryRange::new(0, 2);
        let subtracted = orig.subtract_unary_range(wo_range);
        assert_eq!(3, subtracted.ranges.len());
        assert_eq!(UnaryRange { low: 3, high: 4 }, subtracted.ranges[0]);
        assert_eq!(o2.ranges[1], subtracted.ranges[1]);
        assert_eq!(o2.ranges[2], subtracted.ranges[2]);
    }
    #[test]
    fn test_subtract_unary_range_lowest_higher() {
        let orig = DisjointRange::from_bounds(vec![(0, 4), (6, 10), (12, 16)]);
        let o2 = orig.clone();
        let wo_range = UnaryRange::new(2, 4);
        let subtracted = orig.subtract_unary_range(wo_range);
        assert_eq!(3, subtracted.ranges.len());
        assert_eq!(UnaryRange { low: 0, high: 1 }, subtracted.ranges[0]);
        assert_eq!(o2.ranges[1], subtracted.ranges[1]);
        assert_eq!(o2.ranges[2], subtracted.ranges[2]);
    }
    #[test]
    fn test_subtract_unary_range_lowest_middle() {
        let orig = DisjointRange::from_bounds(vec![(0, 4), (6, 10), (12, 16)]);
        let o2 = orig.clone();
        let wo_range = UnaryRange::new(2, 3);
        let subtracted = orig.subtract_unary_range(wo_range);
        assert_eq!(4, subtracted.ranges.len());
        assert_eq!(UnaryRange { low: 0, high: 1 }, subtracted.ranges[0]);
        assert_eq!(UnaryRange { low: 4, high: 4 }, subtracted.ranges[1]);
        assert_eq!(o2.ranges[1], subtracted.ranges[2]);
        assert_eq!(o2.ranges[2], subtracted.ranges[3]);
    }
    #[test]
    fn test_subtract_unary_range_lower_spanning() {
        let orig = DisjointRange::from_bounds(vec![(0, 4), (6, 10), (12, 16)]);
        let o2 = orig.clone();
        let wo_range = UnaryRange::new(4, 7);
        let subtracted = orig.subtract_unary_range(wo_range);
        assert_eq!(3, subtracted.ranges.len());
        assert_eq!(UnaryRange { low: 0, high: 3 }, subtracted.ranges[0]);
        assert_eq!(UnaryRange { low: 8, high: 10 }, subtracted.ranges[1]);
        assert_eq!(o2.ranges[2], subtracted.ranges[2]);
    }
}
