
#[cfg(test)]
mod lib_test {
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
