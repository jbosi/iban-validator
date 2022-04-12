use wasm_bindgen::prelude::*;
use std::{fmt};

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