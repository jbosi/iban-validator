use std::{str::Chars, fmt};
use wasm_bindgen::prelude::*;

const LETTER_A_VALUE: u16 = 'A' as u16;
const ASCII_OFFSET_TO_ZERO: u16 = 48;
const LETTER_OFFSET_TO_CHECKSUM: u16 = 10;

#[wasm_bindgen]
pub fn validate(raw_input: &str) -> Result<bool, JsError> {
	validate_not_null(raw_input)?;

	let iban_candidate: String = raw_input.chars()
		.filter(|c| !c.is_whitespace())
		.collect();

	validate_length(&iban_candidate)?;
	validate_first_letters(&iban_candidate)?;
	validate_checksum(&iban_candidate)?;

	return Ok(true);
}

fn validate_not_null(iban_candidate: &str) -> Result<bool, JsError> {
	if iban_candidate.is_empty() {
		return Err(JsError::new(&ValidationErrorCodes::E01.to_string()));
	}
	return Ok(true);
}

fn validate_length(iban_candidate: &str) -> Result<bool, JsError> {
	if iban_candidate.len() <= 34 {
		return Ok(true);
	};
	return Err(JsError::new(&ValidationErrorCodes::E02.to_string()));
}

fn validate_first_letters(iban_candidate: &str) -> Result<bool, JsError> {
	let mut chars: Chars = iban_candidate.chars();
	let first_char: char = chars.next().unwrap();
	let second_char: char = chars.next().unwrap();

	if first_char.is_ascii_alphabetic()
		&& first_char.is_uppercase()
		&& second_char.is_ascii_alphabetic()
		&& second_char.is_uppercase() {
		return Ok(true);
	}

	return Err(JsError::new(&ValidationErrorCodes::E03.to_string()));
}

fn validate_checksum(iban_candidate: &str) -> Result<bool, JsError> {
	let iban_without_four_first_block: String = iban_candidate[4..iban_candidate.len()].to_string();
	let first_chars = &iban_candidate[0..4];

	let shuffled_iban = iban_without_four_first_block + first_chars;

	let result: u16 = shuffled_iban.chars().map(|val| {
		if val.is_ascii_alphabetic() {
			return (val as u16) - LETTER_A_VALUE + LETTER_OFFSET_TO_CHECKSUM;
		}
		return (val as u16) - ASCII_OFFSET_TO_ZERO;
	}).sum();

	if (result / 97) == 1 {
		return Ok(true);
	};

	return Err(JsError::new(&ValidationErrorCodes::E04.to_string()));
}

#[wasm_bindgen]
pub enum ValidationErrorCodes {
	E01,
	E02,
	E03,
	E04,
	E05,
	E06
}

impl fmt::Display for ValidationErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationErrorCodes::E01 => write!(f, "E01"),
            ValidationErrorCodes::E02 => write!(f, "E02"),
            ValidationErrorCodes::E03 => write!(f, "E03"),
            ValidationErrorCodes::E04 => write!(f, "E04"),
            ValidationErrorCodes::E05 => write!(f, "E05"),
            ValidationErrorCodes::E06 => write!(f, "E06"),
        }
    }
}