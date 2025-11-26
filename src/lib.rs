#![deny(clippy::all)]

use napi_derive::napi;

#[derive(Debug)]
#[napi(string_enum)]
pub enum Signature {
    Positive,
    Negative,
    Zero,
}

#[derive(Debug)]
#[napi]
pub struct Decimal {
    pub num_of_decimals: u32,
    pub signature: Signature,
    pub digits: Vec<u8>,
}

#[napi]
impl Decimal {
    #[napi]
    pub fn new(number: String) -> Option<Decimal> {
        if !Self::is_valid(&number) {
            return None;
        }

        let first_char: char = number.chars().next().unwrap();
        if first_char == '0' {
            return Some(Decimal {
                num_of_decimals: 0,
                signature: Signature::Zero,
                digits: vec![0],
            });
        }

        let num_of_decimals: u32 = Self::get_num_of_decimals(&number);
        let signature: Signature = if first_char == '-' {
            Signature::Negative
        } else {
            Signature::Positive
        };
        let digits: Vec<u8> = Self::get_digits(&number);

        Some(Decimal {
            num_of_decimals,
            signature,
            digits,
        })
    }

    fn is_valid(number: &str) -> bool {
        if number.chars().count() == 0 {
            return false;
        }

        let first_char: char = number.chars().next().unwrap();
        if first_char != '+' && first_char != '-' && !first_char.is_ascii_digit() {
            return false;
        }

        for (i, c) in number.char_indices() {
            if i == 0 {
                continue;
            }

            if i == 1 && !c.is_ascii_digit() {
                return false;
            }

            if i == number.chars().count() - 1 && !c.is_ascii_digit() {
                return false;
            }

            if c != '.' && !c.is_ascii_digit() {
                return false;
            }
        }

        true
    }

    fn get_num_of_decimals(number: &str) -> u32 {
        let decimals: Option<&str> = number.split(".").nth(1);

        match decimals {
            Some(d) => d.chars().count() as u32,
            None => 0,
        }
    }

    fn get_digits(number: &str) -> Vec<u8> {
        let mut parsed_number: String = number.replace("+", "").replace("-", "").replace(".", "");

        if parsed_number.chars().count() % 2 != 0 {
            parsed_number.insert(0, '0');
        }

        let number_of_bytes: usize = parsed_number.chars().count() / 2;
        let mut digits: Vec<u8> = Vec::with_capacity(number_of_bytes);

        for i in 0..number_of_bytes {
            let index: usize = 2 * (number_of_bytes - i) - 1;

            let low_nibble: u8 = parsed_number.as_bytes()[index] - b'0';
            let high_nibble: u8 = parsed_number.as_bytes()[index - 1] - b'0';

            digits.push((high_nibble << 4) | low_nibble);
        }

        digits
    }
}
