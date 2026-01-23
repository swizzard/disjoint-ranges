# disjoint-ranges

![docs.rs](https://img.shields.io/docsrs/disjoint-ranges)
![Crates.io License](https://img.shields.io/crates/l/disjoint-ranges)


## What
A library providing disjoint ranges, i.e. ranges with gaps in them.

```rust
// checked range creation, ensure low <= high
assert!(DisjointRange::new_single_range(40u8, 10).is_none());

// `_unchecked` alternatives for creation methods
let range = DisjointRange::new_single_range_unchecked('a', 'f');

// check membership
assert!(&range.contains(&'c'));
assert!(!&range.contains(&'h'));
assert!(!&range.contains(&'s'));

// add
let range = DisjointRange::new_single_range_unchecked('a', 'f');
let added = range.add_unary_range(UnaryRange::new_unchecked('q', 'z'));
assert!(&added.contains(&'s'));
assert!(!&range.contains(&'h'));

// subtract
let range = DisjointRange::new_single_range_unchecked(60u8, 120u8);
assert!(!&added.contains(45));
assert!(&added.contains(85));
assert!(&added.contains(95));

range.subtract_unary_range(UnaryRange::new_unchecked(40, 90));
assert!(!&range.contains(45));
assert!(!&range.contains(85));
assert!(&range.contains(95));

// complement
let range = DisjointRange::new_single_range_unchecked(60u16, 120u16);
assert!(&range.contains(70));
assert!(!&range.contains(59));
assert!(!&range.contains(123));

let comp = range.complement();
assert!(!&range.contains(70));
assert!(&range.contains(59));
assert!(&range.contains(123))

// iterate over subranges
let range = DisjointRange::from_bounds_unchecked([(0u8, 9), (20, 40), (90, 120)]);
for subrange in range.iter_ranges() {
    let (low, high) = subrange.as_bounds();
    println!("low {} high {}", low, high);
// prints
// low 0 high 9
// low 20 high 40
// low 90 high 120
```

## Who
[Sam Raker](https://swizzard.pizza "swizzard dot pizza")

No LLMs or similar "AI" technologies have been used in the making of this library.
