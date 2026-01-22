# disjoint-ranges

![docs.rs](https://img.shields.io/docsrs/disjoint-ranges)
![Crates.io License](https://img.shields.io/crates/l/disjoint-ranges)


## What
A library providing disjoint ranges, i.e. ranges with gaps in them.

```rust
// check membership
let range = DisjointRange::new_single_range('a', 'f');
assert!(&range.contains(&'c'));
assert!(!&range.contains(&'h'));
assert!(!&range.contains(&'s'));

// add
let range = DisjointRange::new_single_range('a', 'f');
let added = range.add_unary_range(UnaryRange::new('q', 'z'));
assert!(&added.contains(&'s'));
assert!(!&range.contains(&'h'));

// subtract
let range = DisjointRange::new_single_range(60u8, 120u8);
assert!(!&added.contains(45));
assert!(&added.contains(85));
assert!(&added.contains(95));

range.subtract_unary_range(UnaryRange::new(40, 90));
assert!(!&range.contains(45));
assert!(!&range.contains(85));
assert!(&range.contains(95));

// complement
let range = DisjointRange::new_single_range(60u16, 120u16);
assert!(&range.contains(70));
assert!(!&range.contains(59));
assert!(!&range.contains(123));

let comp = range.complement();
assert!(!&range.contains(70));
assert!(&range.contains(59));
assert!(&range.contains(123))
```

## Who
[Sam Raker](https://swizzard.pizza "swizzard dot pizza")

No LLMs or similar "AI" technologies have been used in the making of this library.
