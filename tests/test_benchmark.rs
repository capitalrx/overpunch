#![feature(test)]

use overpunch::{convert_from_signed_format, convert_to_signed_format};
use rust_decimal::Decimal;
use test::Bencher;

extern crate test;

#[bench]
fn bench_convert_from_signed_format(b: &mut Bencher) {
    b.iter(|| {
        for _i in 1..10000 {
            convert_from_signed_format("123{", "s9(7)v99");
        }
    });
}

#[bench]
fn bench_convert_to_signed_format(b: &mut Bencher) {
    let val = Decimal::from_str_exact("225.8").unwrap();

    b.iter(|| {
        for _i in 1..10000 {
            convert_to_signed_format(val, "s9(7)v99");
        }
    });
}
