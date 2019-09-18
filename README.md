# `associative_cache`

**A generic, fixed-size, associative cache data structure mapping `K` keys to
`V` values.**

[![](https://docs.rs/associative-cache/badge.svg)](https://docs.rs/associative-cache/)
[![](https://img.shields.io/crates/v/associative-cache.svg)](https://crates.io/crates/associative-cache)
[![](https://img.shields.io/crates/d/associative-cache.svg)](https://crates.io/crates/associative-cache)
[![Build Status](https://dev.azure.com/fitzgen/associative-cache/_apis/build/status/fitzgen.associative-cache?branchName=master)](https://dev.azure.com/fitzgen/associative-cache/_build/latest?definitionId=2&branchName=master)

## Capacity

The cache has a constant, fixed-size capacity which is controlled by the `C`
type parameter and the `Capacity` trait. The memory for the cache entries is
eagerly allocated once and never resized.

## Associativity

The cache can be configured as direct-mapped, two-way associative, four-way
associative, etc... via the `I` type parameter and `Indices` trait.

## Replacement Policy

The cache can be configured to replace the least recently used (LRU) entry, or a
random entry via the `R` type parameter and the `Replacement` trait.

## Examples

```rust
use associative_cache::*;

// A two-way associative cache with random replacement mapping
// `String`s to `usize`s.
let cache = AssociativeCache::<
    String,
    usize,
    Capacity512,
    HashTwoWay,
    RandomReplacement
>::default();

// A four-way associative cache with random replacement mapping
// `*mut usize`s to `Vec<u8>`s.
let cache = AssociativeCache::<
    *mut usize,
    Vec<u8>,
    Capacity32,
    PointerFourWay,
    RandomReplacement
>::default();

// An eight-way associative, least recently used (LRU) cache mapping
// `std::path::PathBuf`s to `std::fs::File`s.
let cache = AssociativeCache::<
    std::path::PathBuf,
    WithLruTimestamp<std::fs::File>,
    Capacity128,
    HashEightWay,
    LruReplacement,
>::default();
```
