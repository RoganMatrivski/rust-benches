use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn split(str: &str) {
    let mut split_str = str.split('|');
    let first = split_str.next().unwrap();
    let second = split_str.next().unwrap();

    let _ret = (first, second);
}

fn split_at_find(str: &str) {
    let split_pos = str.find('|').unwrap();
    let (first, second) = str.split_at(split_pos);

    let _ret = (first, second);
}

fn split_at_position(str: &str) {
    let split_pos = str.chars().position(|c| c == '|').unwrap();
    let (first, second) = str.split_at(split_pos);

    let _ret = (first, second);
}

const INPUT_STR: &str = "asdasd|asddsa";

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_str = INPUT_STR.to_string();

    c.bench_with_input(BenchmarkId::new("split", INPUT_STR), &input_str, |b, s| {
        b.iter(|| split(s))
    });

    c.bench_with_input(
        BenchmarkId::new("split_at_find", INPUT_STR),
        &input_str,
        |b, s| b.iter(|| split_at_find(s)),
    );

    c.bench_with_input(
        BenchmarkId::new("split_at_position", INPUT_STR),
        &input_str,
        |b, s| b.iter(|| split_at_position(s)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
