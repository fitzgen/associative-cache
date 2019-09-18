use associative_cache::*;
use criterion::*;

fn run_bench<C: Capacity, I: Indices<usize, C>>(c: &mut Criterion, name: &str) {
    let elems = C::CAPACITY;

    {
        let mut group = c.benchmark_group("Insertion");

        group.bench_function(name, |b| {
            let mut cache = AssociativeCache::<usize, usize, C, I, RoundRobinReplacement>::default();
            let mut iter = (0..elems).cycle();
            b.iter(|| {
                let i = iter.next().unwrap();
                let key = black_box(i);
                let val = black_box(i);
                black_box(cache.insert(key, val));
            })
        });
    }

    {
        let mut group = c.benchmark_group("Query");

        group.bench_function(name, |b| {
            let mut cache = AssociativeCache::<usize, usize, C, I, RoundRobinReplacement>::default();

            for i in 0..elems {
                // Make the cache have a mix of existing and missing entries.
                if i % 2 == 0 {
                    cache.insert(i, i);
                }
            }

            let mut iter = (0..elems).cycle();

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
    direct_mapped_512(Capacity512, DirectMapped);
    two_way_512(Capacity512, TwoWay);
    four_way_512(Capacity512, FourWay);
    eight_way_512(Capacity512, EightWay);
    sixteen_way_512(Capacity512, SixteenWay);
    thirty_two_way_512(Capacity512, ThirtyTwoWay);
}

criterion_main!(benches);
