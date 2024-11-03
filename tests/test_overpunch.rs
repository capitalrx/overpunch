use overpunch::{convert_from_signed_format, convert_to_signed_format};
use rust_decimal::Decimal;
use test_log::test;

#[test]
fn test_convert_from_single_character() {
    let result = convert_from_signed_format("{", "s9(9)v99").unwrap();
    assert_eq!("0.0", format!("{:.1}", result));

    let result = convert_from_signed_format("}", "s9(9)v99").unwrap();
    assert_eq!("-0.0", format!("{:.1}", result));
}

#[test]
fn test_convert_from_single_digit() {
    let result = convert_from_signed_format("5", "s9(9)v99").unwrap();
    assert_eq!("0.05", format!("{:.2}", result));

    let result = convert_from_signed_format("N", "s9(9)v99").unwrap();
    assert_eq!("-0.05", format!("{:.2}", result));
}

#[test]
fn test_convert_from_signed_format_empty_field_returns_none() {
    let result = convert_from_signed_format("", "s9(9)v99");
    assert!(result.is_none());
}

#[test]
fn test_convert_to_single_character() {
    let result =
        convert_to_signed_format(Decimal::from_str_exact("0.0").unwrap(), "s9(9)v99").unwrap();
    assert_eq!("00{", result);

    let result =
        convert_to_signed_format(-Decimal::from_str_exact("0.0").unwrap(), "s9(9)v99").unwrap();
    assert_eq!("00}", result);
}

#[test]
fn test_convert_to_single_digit() {
    let result =
        convert_to_signed_format(Decimal::from_str_exact("0.5").unwrap(), "s9(9)v99").unwrap();
    assert_eq!("05{", result);

    let result =
        convert_to_signed_format(Decimal::from_str_exact("-0.5").unwrap(), "s9(9)v99").unwrap();
    assert_eq!("05}", result);
}

#[test]
fn test_convert_from_signed_format() {
    let value = convert_from_signed_format("2258{", "s9(7)v99").unwrap();
    assert_eq!(value, Decimal::from_str_exact("225.8").unwrap());
    assert_eq!("225.80", format!("{:.2}", value));

    let mut value = convert_from_signed_format("30000", "9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("30.0").unwrap());
    assert_eq!("30.000", format!("{:.3}", value));

    value = convert_from_signed_format("180592", "9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("180.592").unwrap());
    assert_eq!("180.592", format!("{:.3}", value));

    value = convert_from_signed_format("12345G", "s9(9)v99").unwrap();
    assert_eq!(value, Decimal::from_str_exact("1234.57").unwrap());

    value = convert_from_signed_format("{", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0").unwrap());
    assert_eq!("0.000", format!("{:.3}", value));

    value = convert_from_signed_format("N", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("-0.005").unwrap());
    assert_eq!("-0.005", format!("{:.3}", value));

    value = convert_from_signed_format("0N", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("-0.005").unwrap());
    assert_eq!("-0.005", format!("{:.3}", value));

    value = convert_from_signed_format("00N", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("-0.005").unwrap());
    assert_eq!("-0.005", format!("{:.3}", value));

    value = convert_from_signed_format("000N", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("-0.005").unwrap());
    assert_eq!("-0.005", format!("{:.3}", value));

    value = convert_from_signed_format("0000N", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("-0.005").unwrap());
    assert_eq!("-0.005", format!("{:.3}", value));

    value = convert_from_signed_format("00000N", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("-0.005").unwrap());
    assert_eq!("-0.005", format!("{:.3}", value));

    value = convert_from_signed_format("G", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0.007").unwrap());
    assert_eq!("0.007", format!("{:.3}", value));

    value = convert_from_signed_format("0G", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0.007").unwrap());
    assert_eq!("0.007", format!("{:.3}", value));

    value = convert_from_signed_format("00G", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0.007").unwrap());
    assert_eq!("0.007", format!("{:.3}", value));

    value = convert_from_signed_format("000G", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0.007").unwrap());
    assert_eq!("0.007", format!("{:.3}", value));

    value = convert_from_signed_format("0000G", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0.007").unwrap());
    assert_eq!("0.007", format!("{:.3}", value));

    value = convert_from_signed_format("1F", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0.016").unwrap());
    assert_eq!("0.016", format!("{:.3}", value));

    value = convert_from_signed_format("21C", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0.213").unwrap());
    assert_eq!("0.213", format!("{:.3}", value));

    value = convert_from_signed_format("67L", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("-0.673").unwrap());
    assert_eq!("-0.673", format!("{:.3}", value));

    value = convert_from_signed_format("123Q", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("-1.238").unwrap());
    assert_eq!("-1.238", format!("{:.3}", value));

    value = convert_from_signed_format("133I", "s9(7)v999").unwrap();
    assert_eq!(value, Decimal::from_str_exact("1.339").unwrap());
    assert_eq!("1.339", format!("{:.3}", value));

    value = convert_from_signed_format("{", "s9(7)").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0").unwrap());
    assert_eq!("0.000", format!("{:.3}", value));

    value = convert_from_signed_format("}", "s9(7)").unwrap();
    assert_eq!(value, Decimal::from_str_exact("0").unwrap());
    assert_eq!("-0.000", format!("{:.3}", value));

    value = convert_from_signed_format("B", "s9(7)").unwrap();
    assert_eq!(value, Decimal::from_str_exact("2").unwrap());
    assert_eq!("2.000", format!("{:.3}", value));

    value = convert_from_signed_format("K", "s9(7)").unwrap();
    assert_eq!(value, Decimal::from_str_exact("-2").unwrap());
    assert_eq!("-2.000", format!("{:.3}", value));
}

#[test]
fn test_convert_to_signed_format() {
    let mut value =
        convert_to_signed_format(Decimal::from_str_exact("225.8").unwrap(), "s9(7)v99").unwrap();
    assert_eq!(value, "2258{");

    value =
        convert_to_signed_format(Decimal::from_str_exact("225.80").unwrap(), "s9(7)v99").unwrap();
    assert_eq!(value, "2258{");

    value =
        convert_to_signed_format(Decimal::from_str_exact("225.801").unwrap(), "s9(7)v99").unwrap();
    assert_eq!(value, "2258{");

    value = convert_to_signed_format(Decimal::from_str_exact("-12.3450").unwrap(), "s9(7)v9999")
        .unwrap();
    assert_eq!(value, "12345}");

    value = convert_to_signed_format(Decimal::from_str_exact("-12.3451").unwrap(), "s9(7)v9999")
        .unwrap();
    assert_eq!(value, "12345J");

    let mut value =
        convert_to_signed_format(Decimal::from_str_exact("30.000").unwrap(), "9(7)v999").unwrap();
    assert_eq!(value, "3000{");

    value =
        convert_to_signed_format(Decimal::from_str_exact("180.592").unwrap(), "9(7)v999").unwrap();
    assert_eq!(value, "18059B");

    value = convert_to_signed_format(Decimal::from_str_exact("1234.5678").unwrap(), "s9(9)v99")
        .unwrap();
    assert_eq!(value, "12345G");

    value = convert_to_signed_format(Decimal::from_str_exact("0.0008").unwrap(), "s9(9)v99")
        .unwrap();
    assert_eq!(value, "00{");

    value = convert_to_signed_format(Decimal::from_str_exact("0.008").unwrap(), "s9(9)v99")
        .unwrap();
    assert_eq!(value, "00A");

    value = convert_to_signed_format(Decimal::from_str_exact("0.004").unwrap(), "s9(9)v99")
        .unwrap();
    assert_eq!(value, "00{");

    value = convert_to_signed_format(Decimal::from_str_exact("0.08").unwrap(), "s9(9)v99")
        .unwrap();
    assert_eq!(value, "00H");

    value = convert_to_signed_format(Decimal::from_str_exact("-0.008").unwrap(), "s9(9)v99")
        .unwrap();
    assert_eq!(value, "00J");

    value = convert_to_signed_format(Decimal::from_str_exact("-0.004").unwrap(), "s9(9)v99")
        .unwrap();
    assert_eq!(value, "00}");

    value = convert_to_signed_format(Decimal::from_str_exact("0.004").unwrap(), "s9(9)v999")
        .unwrap();
    assert_eq!(value, "000D");

    value = convert_to_signed_format(Decimal::from_str_exact("0.004").unwrap(), "s9(9)v9999")
        .unwrap();
    assert_eq!(value, "0004{");
}
