use std::collections::HashMap;
use std::sync::LazyLock;

use rust_decimal::{Decimal, RoundingStrategy};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OverpunchError {
    #[error("cannot extract from an empty field")]
    EmptyField,

    #[error("invalid character '{0}' when extracting field")]
    InvalidLastChar(char),

    #[error("invalid character '{0}' when formatting field")]
    InvalidFieldFormatting(char),

    #[error("failed to parse result as decimal: {0}")]
    ParseError(String),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct ExtractRef {
    sign: char,
    cent: char,
}

impl ExtractRef {
    fn new(sign: char, cent: char) -> Self {
        ExtractRef { sign, cent }
    }
}

static EXTRACT_REF: LazyLock<HashMap<char, ExtractRef>> = LazyLock::new(|| {
    [
        ('0', ExtractRef::new('+', '0')),
        ('1', ExtractRef::new('+', '1')),
        ('2', ExtractRef::new('+', '2')),
        ('3', ExtractRef::new('+', '3')),
        ('4', ExtractRef::new('+', '4')),
        ('5', ExtractRef::new('+', '5')),
        ('6', ExtractRef::new('+', '6')),
        ('7', ExtractRef::new('+', '7')),
        ('8', ExtractRef::new('+', '8')),
        ('9', ExtractRef::new('+', '9')),
        ('{', ExtractRef::new('+', '0')),
        ('A', ExtractRef::new('+', '1')),
        ('B', ExtractRef::new('+', '2')),
        ('C', ExtractRef::new('+', '3')),
        ('D', ExtractRef::new('+', '4')),
        ('E', ExtractRef::new('+', '5')),
        ('F', ExtractRef::new('+', '6')),
        ('G', ExtractRef::new('+', '7')),
        ('H', ExtractRef::new('+', '8')),
        ('I', ExtractRef::new('+', '9')),
        ('}', ExtractRef::new('-', '0')),
        ('J', ExtractRef::new('-', '1')),
        ('K', ExtractRef::new('-', '2')),
        ('L', ExtractRef::new('-', '3')),
        ('M', ExtractRef::new('-', '4')),
        ('N', ExtractRef::new('-', '5')),
        ('O', ExtractRef::new('-', '6')),
        ('P', ExtractRef::new('-', '7')),
        ('Q', ExtractRef::new('-', '8')),
        ('R', ExtractRef::new('-', '9')),
    ]
    .iter()
    .cloned()
    .collect()
});

static FORMAT_REF: LazyLock<HashMap<ExtractRef, char>> = LazyLock::new(|| {
    [
        (ExtractRef::new('+', '0'), '{'),
        (ExtractRef::new('+', '1'), 'A'),
        (ExtractRef::new('+', '2'), 'B'),
        (ExtractRef::new('+', '3'), 'C'),
        (ExtractRef::new('+', '4'), 'D'),
        (ExtractRef::new('+', '5'), 'E'),
        (ExtractRef::new('+', '6'), 'F'),
        (ExtractRef::new('+', '7'), 'G'),
        (ExtractRef::new('+', '8'), 'H'),
        (ExtractRef::new('+', '9'), 'I'),
        (ExtractRef::new('-', '0'), '}'),
        (ExtractRef::new('-', '1'), 'J'),
        (ExtractRef::new('-', '2'), 'K'),
        (ExtractRef::new('-', '3'), 'L'),
        (ExtractRef::new('-', '4'), 'M'),
        (ExtractRef::new('-', '5'), 'N'),
        (ExtractRef::new('-', '6'), 'O'),
        (ExtractRef::new('-', '7'), 'P'),
        (ExtractRef::new('-', '8'), 'Q'),
        (ExtractRef::new('-', '9'), 'R'),
    ]
    .iter()
    .cloned()
    .collect()
});

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

    format(value, number_of_decimal_places as usize).ok()
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

fn extract(raw: &str, decimals: usize) -> Result<Decimal, OverpunchError> {
    let length = raw.len();
    if length == 0 {
        return Err(OverpunchError::EmptyField);
    }

    let last_char = raw
        .chars()
        .nth(length - 1)
        .ok_or_else(|| OverpunchError::InvalidLastChar(raw.chars().next().unwrap_or(' ')))?;

    let (sign, cent) = EXTRACT_REF
        .get(&last_char)
        .map(|r| (r.sign, r.cent))
        .ok_or(OverpunchError::InvalidLastChar(last_char))?;

    let core = if decimals == 0 {
        raw[..length - 1].to_string()
    } else {
        let padded = format!("{:0>width$}", raw, width = decimals);
        let padded_len = padded.len();

        format!(
            "{}.{}",
            &padded[..padded_len - decimals],
            &padded[padded_len - decimals..padded_len - 1],
        )
    };

    let mut result = String::new();
    result.push(sign);
    result.push_str(core.as_str());
    result.push(cent);

    result
        .parse::<Decimal>()
        .map_err(|_| OverpunchError::ParseError(result))
}

fn format(value: Decimal, decimals: usize) -> Result<String, OverpunchError> {
    let sign = if value.is_sign_negative() { '-' } else { '+' };
    let base_val_str = value
        .abs()
        .round_dp_with_strategy(
            decimals.try_into().unwrap(),
            RoundingStrategy::MidpointAwayFromZero,
        )
        .to_string();

    let parts: Vec<_> = base_val_str.splitn(2, ".").collect();

    let (core, frac) = if parts.len() > 1 {
        (parts[0], parts[1])
    } else {
        (parts[0], "")
    };

    let padded_frac = format!("{:0<width$}", frac, width = decimals);

    let val_str = format!("{}{}", core, padded_frac);

    let last_char = val_str
        .chars()
        .last()
        .ok_or_else(|| OverpunchError::InvalidLastChar(val_str.chars().next().unwrap_or(' ')))?;
    let format_char = FORMAT_REF
        .get(&ExtractRef::new(sign, last_char))
        .ok_or(OverpunchError::InvalidFieldFormatting(last_char))?;

    let mut formatted_val = String::with_capacity(val_str.len());
    formatted_val.push_str(&val_str[..val_str.len() - 1]);
    formatted_val.push(*format_char);

    Ok(formatted_val)
}
