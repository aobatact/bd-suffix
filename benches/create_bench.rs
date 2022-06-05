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
    for (label, target) in get_bench_str(StrSets::Short) {
        unit_set(&mut unit_group, target, label);
    }
    unit_group.finish();
}

fn create_bench_str(c: &mut Criterion) {
    let mut str_group = c.benchmark_group("create_str");
    for (label, target) in get_bench_str(StrSets::Short) {
        str_set(&mut str_group, target, label);
    }
    str_group.finish();
}

fn create_bench_long_sais(c: &mut Criterion) {
    let mut str_group = c.benchmark_group("create_long");
    for (label, target) in get_bench_str(StrSets::Long) {
        unit_sais(&mut str_group, target, label);
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

fn unit_sais(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    target: &'static str,
    label: &'static str,
) {
    bench_inner::<SAISBuilder, _>(group, target, (), label);
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

enum StrSets {
    Short,
    Long
}

fn get_bench_str(s: StrSets) -> &'static [(&'static str, &'static str)] {
    const SHORT_SET : [(&str, &str); 3] =
    [ 
        ("25-rust4", "rust_rust_xx_xx_rust_rust"),
        ("25-abcy",  "abcdefghijklmnopqrstuvwxy"),
        ("100-random", "xsijecvmbnxqynqpguzombqufmwugoayupbzawgymdtqqtojgydgbcdnqsuvvdzsawcyyevwtvadjaoqagoiceparehcixtnrglh"),
    ];
    const LONG_SET : [(&str, &str); 1] =
    [ 
        ("20000-random", include_str!("random_20000.txt")),
    ];
    match s{
        StrSets::Short => {&SHORT_SET},
        StrSets::Long => {&LONG_SET},
    }
}

criterion_group!(benches, create_bench_unit, create_bench_str, create_bench_long_sais);
criterion_main!(benches);
