# disjoint-ranges

## What
A library providing disjoint ranges, i.e. ranges with gaps in them.

```rust
// check membership
let range = DisjointRange::new_single_range('a', 'f');
assert!(&range.contains(&'c'));
assert_false!(&range.contains(&'h'));
assert_false!(&range.contains(&'s'));

// add
let range = DisjointRange::new_single_range('a', 'f');
let added = range.add_unary_range(UnaryRange::new('q', 'z'));
assert!(&added.contains(&'s'));
assert_false!(&range.contains(&'h'));

// subtract
let range = DisjointRange::new_single_range(60u8, 120u8);
assert_false!(&added.contains(45));
assert!(&added.contains(85));
assert!(&added.contains(95));

let sub = range.subtract_unary_range(UnaryRange::new(40, 90));
assert_false!(&added.contains(45));
assert_false!(&added.contains(85));
assert!(&added.contains(95));

// complement
let range = DisjointRange::new_single_range(60u16, 120u16);
assert!(&range.contains(70));
assert_false!(&range.contains(59));
assert_false!(&range.contains(123));

let comp = range.complement();
assert_false!(&range.contains(70));
assert!(&range.contains(59));
assert!(&range.contains(123))
```
