extern crate criterion;
extern crate elliptic_ops;
extern crate num_bigint;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use elliptic_ops::fp::stark::FStark;
use elliptic_ops::fp::nist::Fp;
use elliptic_ops::point::{CurveConfig, Point};
use elliptic_ops::randbiguint::RandBigUint;
use num_bigint::BigUint;

fn bench_curve_ops(c: &mut Criterion) {
    let p1 = Point::<FStark>::generator(); //UNUSED
    let p2 = p1.double();
    // SCALAR = ORD - 1
    let scalar = FStark::n() -  BigUint::from(1u128);
    let table = FStark::power_of_two_table();

    // 1. Benchmark Addition
    c.bench_function("stark_add", |b| {
        b.iter(|| black_box(p1).add(black_box(&p2)))
    });

    // 2. Benchmark Doubling
    c.bench_function("stark_double", |b| {
        b.iter(|| black_box(p1).double())
    });

    // 3. Benchmark Generator Multiplication
    c.bench_function("stark_mul_generator", |b| {
        b.iter(|| Point::mul_gen(black_box(&scalar), black_box(&table)))
    });

    let n = FStark::n();
    let mut rng = rand::thread_rng();
    let a = rng.gen_b_range(&BigUint::from(0u8), &n);
    let k = rng.gen_b_range(&BigUint::from(0u8), &n);
    let a = Point::mul_gen(&a, &table);

    // 4. Benchmark Multiplication
    c.bench_function("stark_mul", |b| {
        b.iter(|| black_box(a).mul(black_box(&k)))
    });

    let p1 = Point::<Fp>::generator(); //UNUSED
    let p2 = p1.double();
    let scalar = Fp::n() -  BigUint::from(1u128);
    let table = Fp::power_of_two_table();

    // 1. Benchmark Addition
    c.bench_function("fp_add", |b| {
        b.iter(|| black_box(p1).add(black_box(&p2)))
    });

    // 2. Benchmark Doubling
    c.bench_function("fp_double", |b| {
        b.iter(|| black_box(p1).double())
    });

    // 3. Benchmark Generator Multiplication
    c.bench_function("fp_mul_generator", |b| {
        b.iter(|| Point::mul_gen(black_box(&scalar), black_box(&table)))
    });

    // 4. Benchmark Multiplication
    let n = Fp::n();
    let mut rng = rand::thread_rng();
    let a = rng.gen_b_range(&BigUint::from(0u8), &n);
    let k = rng.gen_b_range(&BigUint::from(0u8), &n);
    let a = Point::mul_gen(&a, &table);

    // 4. Benchmark Multiplication
    c.bench_function("fp_mul", |b| {
        b.iter(|| black_box(a).mul(black_box(&k)))
    });
}

criterion_group!(benches, bench_curve_ops);
criterion_main!(benches);