use criterion::{criterion_group, criterion_main, Criterion};
use overpunch::{convert_from_signed_format, convert_to_signed_format};
use rust_decimal::Decimal;
use std::hint::black_box;

fn bench_convert_from_signed_format(c: &mut Criterion) {
    c.bench_function("convert_from_signed_format", |b| {
        b.iter(|| {
            convert_from_signed_format("123{", "s9(7)v99");
        })
    });
}

fn bench_convert_to_signed_format(c: &mut Criterion) {
    let val = Decimal::from_str_exact("225.8").unwrap();

    c.bench_function("convert_to_signed_format", |b| {
        b.iter(|| {
            convert_to_signed_format(val, "s9(7)v99");
        })
    });
}

criterion_group!(
    benches,
    bench_convert_from_signed_format,
    bench_convert_to_signed_format,
);
criterion_main!(benches);
