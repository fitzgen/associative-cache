use associative_cache::*;
use criterion::*;

fn run_bench<C: Capacity, I: Indices<*mut u64, C>>(c: &mut Criterion, name: &str) {
    let elems = C::CAPACITY;

    {
        let mut group = c.benchmark_group("Insertion");

        group.bench_function(name, |b| {
            let mut cache =
                AssociativeCache::<*mut u64, usize, C, I, RoundRobinReplacement>::default();
            let mut iter = (0..elems)
                .cycle()
                .map(|i| (i * std::mem::align_of::<u64>()) as *mut u64);
            b.iter(|| {
                let i = iter.next().unwrap();
                let key = black_box(i);
                let val = black_box(i as usize);
                black_box(cache.insert(key, val));
            })
        });
    }

    {
        let mut group = c.benchmark_group("Query");

        group.bench_function(name, |b| {
            let mut cache =
                AssociativeCache::<*mut u64, usize, C, I, RoundRobinReplacement>::default();

            for i in 0..elems {
                // Make the cache have a mix of existing and missing entries.
                if i % 2 == 0 {
                    cache.insert((i * std::mem::align_of::<u64>()) as *mut u64, i);
                }
            }

            let mut iter = (0..elems)
                .cycle()
                .map(|i| (i * std::mem::align_of::<u64>()) as *mut u64);

            b.iter(|| {
                let i = iter.next().unwrap();
                let key = black_box(i);
                black_box(cache.get(&key));
            })
        });
    }
}

macro_rules! define_benches {
    ( $( $name:ident ( $cap:ident, $ind:ident ); )* ) => {
        $(
            fn $name(c: &mut Criterion) {
                run_bench::<$cap, $ind>(c, concat!(stringify!($ind), "-", stringify!($cap)));
            }
        )*

        criterion_group!(benches $( , $name )* );
    }
}

define_benches! {
    hash_direct_mapped_512(Capacity512, HashDirectMapped);
    hash_two_way_512(Capacity512, HashTwoWay);
    hash_four_way_512(Capacity512, HashFourWay);
    hash_eight_way_512(Capacity512, HashEightWay);
    hash_sixteen_way_512(Capacity512, HashSixteenWay);
    hash_thirty_two_way_512(Capacity512, HashThirtyTwoWay);

    pointer_direct_mapped_512(Capacity512, PointerDirectMapped);
    pointer_two_way_512(Capacity512, PointerTwoWay);
    pointer_four_way_512(Capacity512, PointerFourWay);
    pointer_eight_way_512(Capacity512, PointerEightWay);
    pointer_sixteen_way_512(Capacity512, PointerSixteenWay);
    pointer_thirty_two_way_512(Capacity512, PointerThirtyTwoWay);
}

criterion_main!(benches);
