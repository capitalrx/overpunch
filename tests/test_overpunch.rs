use overpunch::{convert_from_signed_format, convert_to_signed_format};
use rust_decimal::Decimal;
use test_log::test;

#[test]
fn test_convert_from_single_character() {
    let result = convert_from_signed_format("{", "s9(9)v99").unwrap();
    assert_eq!("0.0", format!("{:.1}", result));
}

#[test]
fn test_convert_from_signed_format_empty_field_returns_none() {
    let result = convert_from_signed_format("", "s9(9)v99");
    assert!(result.is_none());
}

#[test]
fn test_convert_from_signed_format() {
    let patient_pay_amount = convert_from_signed_format("2258{", "s9(7)v99").unwrap();
    assert_eq!(
        patient_pay_amount,
        Decimal::from_str_exact("225.8").unwrap()
    );
    assert_eq!("225.80", format!("{:.2}", patient_pay_amount));

    let mut quantity_dispensed = convert_from_signed_format("30000", "9(7)v999").unwrap();
    assert_eq!(quantity_dispensed, Decimal::from_str_exact("30.0").unwrap());
    assert_eq!("30.000", format!("{:.3}", quantity_dispensed));

    quantity_dispensed = convert_from_signed_format("180592", "9(7)v999").unwrap();
    assert_eq!(
        quantity_dispensed,
        Decimal::from_str_exact("180.592").unwrap()
    );
    assert_eq!("180.592", format!("{:.3}", quantity_dispensed));
}

#[test]
fn test_convert_to_signed_format() {
    let mut patient_pay_amount =
        convert_to_signed_format(Decimal::from_str_exact("225.80").unwrap(), "s9(7)v99").unwrap();
    assert_eq!(patient_pay_amount, "2258{");

    patient_pay_amount =
        convert_to_signed_format(Decimal::from_str_exact("-12.3450").unwrap(), "s9(7)v9999")
            .unwrap();
    assert_eq!(patient_pay_amount, "12345}");

    let mut quantity_dispensed =
        convert_to_signed_format(Decimal::from_str_exact("30.000").unwrap(), "9(7)v999").unwrap();
    assert_eq!(quantity_dispensed, "3000{");

    quantity_dispensed =
        convert_to_signed_format(Decimal::from_str_exact("180.592").unwrap(), "9(7)v999").unwrap();
    assert_eq!(quantity_dispensed, "18059B");
}
