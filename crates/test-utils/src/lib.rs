pub use associative_cache::*;

use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum MethodCall {
    Insert,
    Remove,
}

impl Arbitrary for MethodCall {
    fn arbitrary<G>(g: &mut G) -> Self
    where
        G: Gen,
    {
        match g.gen_range(0, 2) {
            0 => MethodCall::Insert,
            1 => MethodCall::Remove,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MethodCalls {
    calls: Vec<MethodCall>,
    entries: Vec<Entry>,
}

// NB: Entry contains a `*mut u64` but we never deref it, its just there to be
// able to test `Pointer*Way`.
unsafe impl Send for MethodCalls {}

impl Arbitrary for MethodCalls {
    fn arbitrary<G>(g: &mut G) -> Self
    where
        G: Gen,
    {
        let calls: Vec<MethodCall> = Arbitrary::arbitrary(g);

        let entries: HashMap<usize, usize> = Arbitrary::arbitrary(g);
        let entries: Vec<Entry> = entries
            .into_iter()
            .map(<Entry as From<(usize, usize)>>::from)
            .collect();

        MethodCalls { calls, entries }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let og_calls = self.calls.clone();
        let og_entries = self.entries.clone();

        let shrink_calls = self.calls.shrink();
        let shrink_entries = self
            .entries
            .iter()
            .map(|e| (e.key as usize / std::mem::align_of::<u64>(), e.val))
            .collect::<Vec<_>>()
            .shrink()
            .map(|vs| {
                vs.into_iter()
                    .map(<Entry as From<(usize, usize)>>::from)
                    .collect::<Vec<Entry>>()
            });

        let shrinks = shrink_calls
            .zip(shrink_entries)
            .flat_map(move |(calls, entries)| {
                vec![
                    MethodCalls {
                        calls: calls.clone(),
                        entries: entries.clone(),
                    },
                    MethodCalls {
                        calls: og_calls.clone(),
                        entries,
                    },
                    MethodCalls {
                        calls,
                        entries: og_entries.clone(),
                    },
                ]
            });

        Box::new(shrinks) as _
    }
}

macro_rules! bail {
    ( $($args:expr),* ) => {
        {
            let msg = format!($($args),*);
            eprintln!("error: {}", msg);
            return Err(msg);
        }
    }
}

impl MethodCalls {
    pub fn run<C, I, R>(self) -> Result<(), String>
    where
        C: Capacity,
        I: Indices<*mut u64, C>,
        R: Replacement<usize, C> + Default,
    {
        let MethodCalls { calls, entries } = self;
        let mut cache = AssociativeCache::<*mut u64, usize, C, I, R>::default();
        let mut expected = HashMap::<*mut u64, usize>::new();

        for (method, entry) in calls.into_iter().zip(entries.into_iter()) {
            if cache.len() != expected.len() {
                bail!("cache length mismatch");
            }

            for (expected_key, expected_value) in &expected {
                match cache.get(expected_key) {
                    Some(v) if v == expected_value => continue,
                    otherwise => bail!(
                        "expected {:?}; found {:?}",
                        (expected_key, expected_value),
                        otherwise
                    ),
                }
            }

            for (actual_key, actual_value) in cache.iter() {
                match expected.get(actual_key) {
                    Some(v) if v == actual_value => continue,
                    otherwise => bail!(
                        "expected {:?}; found {:?}",
                        otherwise,
                        (actual_key, actual_value)
                    ),
                }
            }

            match method {
                MethodCall::Insert => {
                    match (
                        cache.insert(entry.key, entry.val),
                        expected.insert(entry.key, entry.val),
                    ) {
                        (None, None) => continue,
                        (Some((k, v)), Some(val)) if k == entry.key && v == val => continue,
                        (Some((k, v)), None) => {
                            if k != entry.key && Some(v) == expected.remove(&k) {
                                continue;
                            }
                            bail!("replaced unknown entry on insert: {:?}", (k, v));
                        }
                        otherwise => {
                            bail!("cache mismatch on insert: {:?}", otherwise);
                        }
                    }
                }
                MethodCall::Remove => {
                    match (cache.remove(&entry.key), expected.remove(&entry.key)) {
                        (Some(v), Some(val)) if v == val => continue,
                        (None, None) => continue,
                        otherwise => {
                            bail!("cache mismatch on delete: {:?}", otherwise);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub key: *mut u64,
    pub val: usize,
}

impl From<(usize, usize)> for Entry {
    fn from((key, val): (usize, usize)) -> Self {
        let key = key.wrapping_mul(std::mem::align_of::<u64>()) as *mut u64;
        Entry { key, val }
    }
}

#[cfg(test)]
mod quickchecks {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn test_pointer_two_way(test: MethodCalls) -> Result<(), String> {
            test.run::<Capacity4, PointerTwoWay, RoundRobinReplacement>()
        }

        fn test_pointer_four_way(test: MethodCalls) -> Result<(), String> {
            test.run::<Capacity8, PointerFourWay, RoundRobinReplacement>()
        }

        fn test_hash_two_way(test: MethodCalls) -> Result<(), String> {
            test.run::<Capacity4, HashTwoWay, RoundRobinReplacement>()
        }

        fn test_hash_four_way(test: MethodCalls) -> Result<(), String> {
            test.run::<Capacity8, HashFourWay, RoundRobinReplacement>()
        }
    }
}
