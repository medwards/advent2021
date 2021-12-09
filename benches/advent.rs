use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent2021::{get_day, MAX_DAY};

fn medwards_solvers_benchmark(c: &mut Criterion) {
    (1..=MAX_DAY).map(|day| day.to_string()).for_each(|day| {
        let (day, input_path, part_one, part_two) =
            get_day(day.as_str()).expect("Invalid day included in benchmark");

        let contents = read_to_string(input_path)
            .unwrap_or_else(|e| panic!("Unable to read from {} - {}", input_path, e));

        c.bench_function(format!("Day {}, Part One", day).as_str(), |b| {
            b.iter(|| part_one(black_box(contents.as_str())).unwrap())
        });

        c.bench_function(format!("Day {}, Part Two", day).as_str(), |b| {
            b.iter(|| part_two(black_box(contents.as_str())).unwrap())
        });
    });
}

fn jhorstmann_solvers_benchmark(c: &mut Criterion) {
    let solvers = [6];
    solvers.iter().map(|day| day.to_string()).for_each(|day| {
        let day = format!("jhorstmann::{}", day);
        let (day, input_path, part_one, part_two) =
            get_day(day.as_str()).expect("Invalid day included in benchmark");

        let contents = read_to_string(input_path)
            .unwrap_or_else(|e| panic!("Unable to read from {} - {}", input_path, e));

        c.bench_function(format!("Day {}, Part One", day).as_str(), |b| {
            b.iter(|| part_one(black_box(contents.as_str())).unwrap())
        });

        c.bench_function(format!("Day {}, Part Two", day).as_str(), |b| {
            b.iter(|| part_two(black_box(contents.as_str())).unwrap())
        });
    });
}

criterion_group!(
    benches,
    medwards_solvers_benchmark,
    jhorstmann_solvers_benchmark
);
criterion_main!(benches);
