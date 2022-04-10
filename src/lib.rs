use std::{str::Chars};
use wasm_bindgen::prelude::*;

const LETTER_A_VALUE: u16 = 'A' as u16;
const ASCII_OFFSET_TO_ZERO: u16 = 48;
const LETTER_OFFSET_TO_CHECKSUM: u16 = 10;

#[wasm_bindgen]
pub fn validate(_raw_input: &str) -> String {
	let iban_candidate: String = _raw_input.chars()
		.filter(|c| !c.is_whitespace())
		.collect();

	let is_valid: bool = validate_length(&iban_candidate)
		&& validate_first_letters(&iban_candidate)
		&& validate_checksum(&iban_candidate);

	if is_valid {
		return "L'iban est valide".to_string();
	} else {
		return "L'iban est invalide".to_string();
	}
}

fn validate_length(iban_candidate: &str) -> bool {
	return iban_candidate.len() <= 34;
}

fn validate_first_letters(iban_candidate: &str) -> bool {
	let mut chars: Chars = iban_candidate.chars();
	let first_char: char = chars.next().unwrap();
	let second_char: char = chars.next().unwrap();

	return first_char.is_ascii_alphabetic()
		&& first_char.is_uppercase()
		&& second_char.is_ascii_alphabetic()
		&& second_char.is_uppercase();
}

fn validate_checksum(iban_candidate: &str) -> bool {
	let iban_without_four_first_block: String = iban_candidate[4..iban_candidate.len()].to_string();
	let first_chars = &iban_candidate[0..4];

	let shuffled_iban = iban_without_four_first_block + first_chars;

	let result: u16 = shuffled_iban.chars().map(|val| {
		if val.is_ascii_alphabetic() {
			return (val as u16) - LETTER_A_VALUE + LETTER_OFFSET_TO_CHECKSUM;
		}
		return (val as u16) - ASCII_OFFSET_TO_ZERO;
	}).sum();

	return (result / 97) == 1;
}
