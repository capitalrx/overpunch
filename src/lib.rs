use memchr::memchr;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use thiserror::Error;
use std::mem::ManuallyDrop;

#[derive(Error, Debug)]
pub enum OverpunchError {
    #[error("cannot extract from an empty field")]
    EmptyField,

    #[error("failed to parse result as decimal: {0}")]
    ParseError(String),

    #[error("failed with overflow while serializing value: {0}")]
    OverflowError(String),
}

/// Returns a `str` serialized from a `Decimal` to the appropriate signed overpunch respresentation.
///
/// # Arguments
///
/// * `value` - The `Decimal` value to serialize.
/// * `field_format` - The signed overpunch picture format.
///
/// # Example
///
/// ```
/// # use overpunch::convert_to_signed_format;
/// # use rust_decimal::Decimal;
///
/// let formatted = convert_to_signed_format(Decimal::from_str_exact("225.8").unwrap(), "s9(7)v99").unwrap();
/// assert_eq!(formatted, "2258{");
/// ```
pub fn convert_to_signed_format(value: Decimal, field_format: &str) -> Option<String> {
    let number_of_decimal_places = memchr(b'v', field_format.as_bytes())
        .map_or(0, |pos| field_format[pos + 1..].len());

    format(value, number_of_decimal_places).ok()
}

/// Returns a `Decimal` parsed from an appropriate signed overpunch respresentation.
///
/// # Arguments
///
/// * `value` - The signed overpunch representation.
/// * `field_format` - The signed overpunch picture format.
///
/// # Example
///
/// ```
/// # use overpunch::convert_from_signed_format;
/// # use rust_decimal::Decimal;
///
/// let number = convert_from_signed_format("2258{", "s9(7)v99").unwrap();
/// assert_eq!(number, Decimal::from_str_exact("225.8").unwrap());
/// ```
pub fn convert_from_signed_format(value: &str, field_format: &str) -> Option<Decimal> {
    let number_of_decimal_places = memchr(b'v', field_format.as_bytes())
        .map_or(0, |pos| field_format[pos + 1..].len());

    extract(value, number_of_decimal_places).ok()
}

/// Returns a `Decimal` parsed from an appropriate signed overpunch respresentation.
///
/// # Arguments
///
/// * `value` - The signed overpunch representation.
/// * `decimals` - The number of digits following the decimal point that this value has.
///
/// # Example
///
/// ```
/// # use overpunch::extract;
/// # use rust_decimal::Decimal;
///
/// let number = extract("2258{", 2).unwrap();
/// assert_eq!(number, Decimal::from_str_exact("225.8").unwrap());
/// ```
pub fn extract(raw: &str, decimals: usize) -> Result<Decimal, OverpunchError> {
    if raw.is_empty() {
        return Err(OverpunchError::EmptyField);
    }

    let mut val: i64 = 0;
    let mut last_char = b'0';

    for c in raw.bytes() {
        let char_val: i64 = match c {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            b'{' => 0,
            b'A' => 1,
            b'B' => 2,
            b'C' => 3,
            b'D' => 4,
            b'E' => 5,
            b'F' => 6,
            b'G' => 7,
            b'H' => 8,
            b'I' => 9,
            b'}' => 0,
            b'J' => 1,
            b'K' => 2,
            b'L' => 3,
            b'M' => 4,
            b'N' => 5,
            b'O' => 6,
            b'P' => 7,
            b'Q' => 8,
            b'R' => 9,
            _ => return Err(OverpunchError::ParseError(raw.to_string())),
        };

        last_char = c;

        val = val * 10 + char_val;
    }

    let sign: i64 = match last_char {
        b'}' | b'J' | b'K' | b'L' | b'M' | b'N' | b'O' | b'P' | b'Q' | b'R' => -1,
        _ => 1,
    };

    let extracted = if sign == -1 {
        -Decimal::new(val, decimals as u32)
    } else {
        Decimal::new(val, decimals as u32)
    };

    Ok(extracted)
}

/// Returns a `str` serialized from a `Decimal` to the appropriate signed overpunch respresentation.
///
/// # Arguments
///
/// * `value` - The `Decimal` value to serialize.
/// * `decimals` - The number of digits following the decimal point that the signed overpunch
///   picture implies.
///
/// # Example
///
/// ```
/// # use overpunch::format;
/// # use rust_decimal::Decimal;
///
/// let formatted = format(Decimal::from_str_exact("225.8").unwrap(), 2).unwrap();
/// assert_eq!(formatted, "2258{");
/// ```
pub fn format(value: Decimal, decimals: usize) -> Result<String, OverpunchError> {
    let is_negative: bool = value.is_sign_negative();

    let scale_factor: Decimal = Decimal::new(10_i64.pow(decimals.try_into().unwrap()), 0);

    let mut working_value = value.abs();
    working_value.rescale(decimals.try_into().unwrap());

    let mut as_int: i64 = match (working_value * scale_factor).to_i64() {
        Some(valid_i64) => valid_i64,
        None => return Err(OverpunchError::OverflowError(value.to_string())),
    };

    let mut v = Vec::with_capacity(10);

    let mut last_digit = as_int % 10;
    as_int /= 10;

    let mut c = match (is_negative, last_digit) {
        (false, 0) => b'{',
        (false, 1) => b'A',
        (false, 2) => b'B',
        (false, 3) => b'C',
        (false, 4) => b'D',
        (false, 5) => b'E',
        (false, 6) => b'F',
        (false, 7) => b'G',
        (false, 8) => b'H',
        (false, 9) => b'I',
        (true, 0) => b'}',
        (true, 1) => b'J',
        (true, 2) => b'K',
        (true, 3) => b'L',
        (true, 4) => b'M',
        (true, 5) => b'N',
        (true, 6) => b'O',
        (true, 7) => b'P',
        (true, 8) => b'Q',
        (true, 9) => b'R',
        _ => unreachable!(),
    };

    v.push(c);

    while as_int > 0 {
        last_digit = as_int % 10;
        as_int /= 10;

        c = match last_digit {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => b'3',
            4 => b'4',
            5 => b'5',
            6 => b'6',
            7 => b'7',
            8 => b'8',
            9 => b'9',
            _ => unreachable!(),
        };

        v.push(c);
    }

    while v.len() < decimals + 1 {
        v.push(b'0');
    }

    v.reverse();
    let mut mdv = ManuallyDrop::new(v);
    let formatted = unsafe {
        String::from_raw_parts(mdv.as_mut_ptr(), mdv.len(), mdv.capacity())
    };

    Ok(formatted)
}
