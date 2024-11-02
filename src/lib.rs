use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use thiserror::Error;

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
    let number_of_decimal_places = if let Some(pos) = field_format.find('v') {
        field_format[pos + 1..].len()
    } else {
        0
    };

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
    let number_of_decimal_places = if let Some(pos) = field_format.find('v') {
        field_format[pos + 1..].len()
    } else {
        0
    };

    extract(value, number_of_decimal_places).ok()
}

pub fn extract(raw: &str, decimals: usize) -> Result<Decimal, OverpunchError> {
    let length = raw.len();
    if length == 0 {
        return Err(OverpunchError::EmptyField);
    }

    let mut val: i64 = 0;

    let mut sign: i64 = 1;

    for c in raw.chars() {
        let char_val: i64 = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            '{' => 0,
            'A' => 1,
            'B' => 2,
            'C' => 3,
            'D' => 4,
            'E' => 5,
            'F' => 6,
            'G' => 7,
            'H' => 8,
            'I' => 9,
            '}' => 0,
            'J' => 1,
            'K' => 2,
            'L' => 3,
            'M' => 4,
            'N' => 5,
            'O' => 6,
            'P' => 7,
            'Q' => 8,
            'R' => 9,
            _ => return Err(OverpunchError::ParseError(raw.to_string())),
        };

        sign = match c {
            '}' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' => -1,
            _ => 1,
        };

        val = val * 10 + char_val;
        if sign < 0 {
            val *= sign;
        }
    }

    let scale = if decimals > length {
        (decimals - length) as u32
    } else {
        decimals as u32
    };

    let extracted = if val == 0 && sign == -1 {
        -Decimal::new(val, scale)
    } else {
        Decimal::new(val, scale)
    };

    Ok(extracted)
}

pub fn format(value: Decimal, decimals: usize) -> Result<String, OverpunchError> {
    let is_negative: bool = value.is_sign_negative();

    let scale_factor: Decimal = Decimal::new(10_i64.pow(decimals.try_into().unwrap()), 0);

    let mut working_value = value.abs();
    working_value.rescale(decimals.try_into().unwrap());

    let mut as_int: i64 = match (working_value * scale_factor).to_i64() {
        Some(valid_i64) => valid_i64,
        None => return Err(OverpunchError::OverflowError(value.to_string())),
    };

    let mut v: Vec<char> = Vec::with_capacity(10);

    let mut last_digit = as_int % 10;
    as_int /= 10;

    let mut c = match (is_negative, last_digit) {
        (false, 0) => '{',
        (false, 1) => 'A',
        (false, 2) => 'B',
        (false, 3) => 'C',
        (false, 4) => 'D',
        (false, 5) => 'E',
        (false, 6) => 'F',
        (false, 7) => 'G',
        (false, 8) => 'H',
        (false, 9) => 'I',
        (true, 0) => '}',
        (true, 1) => 'J',
        (true, 2) => 'K',
        (true, 3) => 'L',
        (true, 4) => 'M',
        (true, 5) => 'N',
        (true, 6) => 'O',
        (true, 7) => 'P',
        (true, 8) => 'Q',
        (true, 9) => 'R',
        _ => unreachable!(),
    };

    v.push(c);

    while as_int > 0 {
        last_digit = as_int % 10;
        as_int /= 10;

        c = match last_digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => unreachable!(),
        };

        v.push(c);
    }

    while v.len() < decimals + 1 {
        v.push('0');
    }

    v.reverse();

    Ok(String::from_iter(v))
}
