use std::{str::Chars, fmt};
use wasm_bindgen::prelude::*;

const LETTER_A_VALUE: u128 = 'A' as u128;
const ASCII_OFFSET_TO_ZERO: u128 = 48;
const LETTER_OFFSET_TO_CHECKSUM: u128 = 10;

#[wasm_bindgen]
pub fn validate(raw_input: &str) -> Result<bool, String> {
	validate_not_null(raw_input)?;
	
	let iban_candidate: String = raw_input.chars()
	.filter(|c| !c.is_whitespace())
	.collect();
	
	validate_length(&iban_candidate)?;
	validate_first_letters(&iban_candidate)?;
	validate_checksum(&iban_candidate)?;
	
	return Ok(true);
}

fn validate_not_null(iban_candidate: &str) -> Result<bool, String> {
	if iban_candidate.is_empty() {
		return Err(ValidationErrorCodes::ErrorNullValue.to_string());
	}
	return Ok(true);
}

fn validate_length(iban_candidate: &str) -> Result<bool, String> {
	if iban_candidate.len() < 15 {
		return Err(ValidationErrorCodes::ErrorMinLength.to_string());
	};
	if iban_candidate.len() <= 34 {
		return Ok(true);
	};
	return Err(ValidationErrorCodes::ErrorMaxLength.to_string());
}

fn validate_first_letters(iban_candidate: &str) -> Result<bool, String> {
	let mut chars: Chars = iban_candidate.chars();
	let first_char: char = chars.next().unwrap();
	let second_char: char = chars.next().unwrap();
	
	if first_char.is_ascii_alphabetic()
		&& first_char.is_uppercase()
		&& second_char.is_ascii_alphabetic()
		&& second_char.is_uppercase() {
		return Ok(true);
	}
	
	return Err(ValidationErrorCodes::ErrorFirstTwoLetters.to_string());
}

fn validate_checksum(iban_candidate: &str) -> Result<bool, String> {
	let iban_without_four_first_block: String = iban_candidate[4..iban_candidate.len()].to_string();
	let first_chars = &iban_candidate[0..4];
	
	let shuffled_iban = iban_without_four_first_block + first_chars;
	
	let result: String = shuffled_iban.chars().map(|val| {
		let number;
		if val.is_ascii_alphabetic() {
			number = (val as u128) - LETTER_A_VALUE + LETTER_OFFSET_TO_CHECKSUM;
		} else {
			number = (val as u128) - ASCII_OFFSET_TO_ZERO;
		}
		return number.to_string();
	}).collect();
	
	if result.parse::<u128>().unwrap().rem_euclid(97) == 1 {
		return Ok(true);
	};
	
	return Err(ValidationErrorCodes::ErrorChecksum.to_string());
}

#[cfg(test)]
mod tests {
	use crate::{validate, ValidationErrorCodes};
	const VALID_IBAN_1: &str = "FR7630006000011234567890189";
	const VALID_IBAN_2: &str = "DE75 5121 0800 1245 1261 99";
	const VALID_IBAN_3: &str = "GB33BUKB20201555555555";
	
	#[test]
	fn should_error_for_min_lenth() {
		let validation = validate("FR7630006");
		assert_eq!(ValidationErrorCodes::ErrorMinLength.to_string(), validation.unwrap_err());
	}

	#[test]
	fn should_error_for_max_lenth() {
		let validation = validate("FR7630006000011234567890189 FR7630006000011234567890189 FR7630006000011234567890189");
		assert_eq!(ValidationErrorCodes::ErrorMaxLength.to_string(), validation.unwrap_err());
	}

	#[test]
	fn should_error_if_no_input_value() {
		let validation = validate("");
		assert_eq!(ValidationErrorCodes::ErrorNullValue.to_string(), validation.unwrap_err());
	}

	#[test]
	fn should_error_if_invalid_checksum() {
		let validation = validate("FR7630006000011234567890183");
		assert_eq!(ValidationErrorCodes::ErrorChecksum.to_string(), validation.unwrap_err());
	}

	#[test]
	fn should_error_if_invalid_two_first_char() {
		let validation = validate("F37630006000011234567890189");
		assert_eq!(ValidationErrorCodes::ErrorFirstTwoLetters.to_string(), validation.unwrap_err());
	}

	#[test]
	fn should_return_valid() {
		let validation_1 = validate(VALID_IBAN_1);
		let validation_2 = validate(VALID_IBAN_2);
		let validation_3 = validate(VALID_IBAN_3);
		assert_eq!(true, validation_1.unwrap());
		assert_eq!(true, validation_2.unwrap());
		assert_eq!(true, validation_3.unwrap());
	}
}

#[wasm_bindgen]
pub enum ValidationErrorCodes {
	ErrorNullValue,
	ErrorMaxLength,
	ErrorFirstTwoLetters,
	ErrorChecksum,
	ErrorMinLength,
	E06
}

impl fmt::Display for ValidationErrorCodes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ValidationErrorCodes::ErrorNullValue => write!(f, "ErrorNullValue"),
			ValidationErrorCodes::ErrorMaxLength => write!(f, "ErrorMaxLength"),
			ValidationErrorCodes::ErrorFirstTwoLetters => write!(f, "ErrorFirstTwoLetters"),
			ValidationErrorCodes::ErrorChecksum => write!(f, "ErrorChecksum"),
			ValidationErrorCodes::ErrorMinLength => write!(f, "ErrorMinLength"),
			ValidationErrorCodes::E06 => write!(f, "E06"),
		}
	}
}