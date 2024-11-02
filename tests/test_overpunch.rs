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
    assert_eq!("0.5", format!("{:.1}", result));

    let result = convert_from_signed_format("N", "s9(9)v99").unwrap();
    assert_eq!("-0.5", format!("{:.1}", result));
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
}
