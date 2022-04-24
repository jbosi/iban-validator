# iban-validator
An iban validator containing rust generated WebAssembly

## Quick start

* Install the package as a JS dependency `npm i iban-validator`
* Import the validate function ex :
```typescript
import { validate } from 'iban-validator';
```
* Use it as any js function :
```typescript
try {
	this.isValid = validate(value)
} catch (e) {
	// Handle error here
}
```

## Error list :
* `ErrorNullValue` : provided value is null
* `ErrorMaxLength`: the value is longer than 35 char
* `ErrorFirstTwoLetters`: the first two letters should be strings (and match any country code - not implemented yet)
* `ErrorChecksum` : the checksum is invalid
* `ErrorMinLength` : the value is shorter than 15 char
