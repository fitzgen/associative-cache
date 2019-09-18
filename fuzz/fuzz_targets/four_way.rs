#![no_main]

use associative_cache_test_utils::*;
use bufrng::BufRng;
use libfuzzer_sys::fuzz_target;
use quickcheck::Arbitrary;

fuzz_target!(|data: &[u8]| {
    let mut gen = quickcheck::StdGen::new(BufRng::new(data), std::cmp::max(data.len(), 1));
    let test = MethodCalls::arbitrary(&mut gen);
    if let Err(e) = test.run::<Capacity32, HashFourWay, RoundRobinReplacement>() {
        panic!("error: {}", e);
    }
});
