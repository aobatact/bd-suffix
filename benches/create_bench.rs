use std::any::type_name;

use bd_suffix::gens::{
    builders::*,
    modes::{IndexMode, StrIndex},
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
// use mycrate::fibonacci;

// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

fn create_bench_unit(c: &mut Criterion) {
    let mut unit_group = c.benchmark_group("create_unit");
    for (label, target) in get_bench_str() {
        unit_set(&mut unit_group, target, label);
    }
    unit_group.finish();
}

fn create_bench_str(c: &mut Criterion) {
    let mut str_group = c.benchmark_group("create_str");
    for (label, target) in get_bench_str() {
        str_set(&mut str_group, target, label);
    }
    str_group.finish();
}

fn full_set(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    target: &'static str,
    label: &'static str,
) {
    bench_inner::<NaiveBuilder, _>(group, target, (), label);
    bench_inner::<NaiveBuilder, _>(group, target, StrIndex, label);
    bench_inner::<BucketBuilder, _>(group, target, (), label);
    bench_inner::<BucketBuilder, _>(group, target, StrIndex, label);
    bench_inner::<TwoStageBuilder, _>(group, target, (), label);
    bench_inner::<TwoStageBuilder, _>(group, target, StrIndex, label);
    bench_inner::<TwoStageBuilderU8, _>(group, target, (), label);
    bench_inner::<TwoStageBuilderU8, _>(group, target, StrIndex, label);
    bench_inner::<SAISBuilder, _>(group, target, (), label);
    bench_inner::<SAISBuilder, _>(group, target, StrIndex, label);
    bench_inner::<SAISBuilderU8, _>(group, target, (), label);
    bench_inner::<SAISBuilderU8, _>(group, target, StrIndex, label);
}

fn unit_set(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    target: &'static str,
    label: &'static str,
) {
    bench_inner::<NaiveBuilder, _>(group, target, (), label);
    bench_inner::<BucketBuilder, _>(group, target, (), label);
    bench_inner::<TwoStageBuilder, _>(group, target, (), label);
    bench_inner::<TwoStageBuilderU8, _>(group, target, (), label);
    bench_inner::<SAISBuilder, _>(group, target, (), label);
    bench_inner::<SAISBuilderU8, _>(group, target, (), label);
}

fn str_set(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    target: &'static str,
    label: &'static str,
) {
    bench_inner::<NaiveBuilder, _>(group, target, StrIndex, label);
    bench_inner::<BucketBuilder, _>(group, target, StrIndex, label);
    bench_inner::<TwoStageBuilder, _>(group, target, StrIndex, label);
    bench_inner::<TwoStageBuilderU8, _>(group, target, StrIndex, label);
    bench_inner::<SAISBuilder, _>(group, target, StrIndex, label);
    bench_inner::<SAISBuilderU8, _>(group, target, StrIndex, label);
}

fn bench_inner<'a, B: Builder<&'a str, u8, Im>, Im: IndexMode<u8> + Copy>(
    group: &'a mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    target: &'a str,
    im: Im,
    label: &'static str,
) {
    group.bench_with_input(
        BenchmarkId::new(
            type_name::<B>().rsplit_once("::").unwrap().1.to_string(),
            label,
        ),
        &target,
        |b, i| {
            b.iter(|| {
                black_box(B::build(&i, im));
            })
        },
    );
}

fn get_bench_str() -> impl IntoIterator<Item = (&'static str, &'static str)> {
    [
        ("25-rust4", "rust_rust_xx_xx_rust_rust"),
        ("25-abcy",  "abcdefghijklmnopqrstuvwxy"),
        ("100-random", "xsijecvmbnxqynqpguzombqufmwugoayupbzawgymdtqqtojgydgbcdnqsuvvdzsawcyyevwtvadjaoqagoiceparehcixtnrglh"),
    ]
}

criterion_group!(benches, create_bench_unit, create_bench_str);
criterion_main!(benches);
