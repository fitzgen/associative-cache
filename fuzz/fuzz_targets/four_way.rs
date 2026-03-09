#![no_main]

use associative_cache_test_utils::*;
use libfuzzer_sys::fuzz_target;
use quickcheck::{Arbitrary, Gen};

fuzz_target!(|input: (u8, u64)| {
    let (size, seed) = input;
    let mut gen = Gen::from_size_and_seed(size.max(1) as usize, seed);
    let test = MethodCalls::arbitrary(&mut gen);
    if let Err(e) = test.run::<Capacity32, HashFourWay, RoundRobinReplacement>() {
        panic!("error: {}", e);
    }
});
