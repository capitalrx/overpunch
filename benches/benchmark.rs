use criterion::{black_box, criterion_group, criterion_main, Criterion};
use overpunch::{convert_from_signed_format, convert_to_signed_format, extract, format};
use rust_decimal::Decimal;

fn bench_convert_from_signed_format(c: &mut Criterion) {
    c.bench_function("convert_from_signed_format", |b| {
        b.iter(|| {
            black_box(convert_from_signed_format(
                black_box("123{"),
                black_box("s9(7)v99"),
            ));
        })
    });
}

fn bench_convert_to_signed_format(c: &mut Criterion) {
    let val = Decimal::from_str_exact("225.8").unwrap();

    c.bench_function("convert_to_signed_format", |b| {
        b.iter(|| {
            black_box(convert_to_signed_format(
                black_box(val),
                black_box("s9(7)v99"),
            ));
        })
    });
}

fn bench_extract(c: &mut Criterion) {
    c.bench_function("extract", |b| {
        b.iter(|| {
            black_box(extract(black_box("123{"), black_box(2))).unwrap();
        })
    });
}

fn bench_format(c: &mut Criterion) {
    let val = Decimal::from_str_exact("225.8").unwrap();

    c.bench_function("format", |b| {
        b.iter(|| {
            black_box(format(black_box(val), black_box(2))).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_convert_from_signed_format,
    bench_convert_to_signed_format,
    bench_extract,
    bench_format,
);
criterion_main!(benches);
