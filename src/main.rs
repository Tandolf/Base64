use core::fmt;
use std::env;

use regex::Regex;

#[derive(Debug, PartialEq, PartialOrd)]
enum DecodeError {
    LengthError,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::LengthError => write!(f, "Non valid length"),
        }
    }
}

struct Base64;

const BASE64_CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

impl Base64 {
    fn encode<T: AsRef<str>>(input: T) -> String {
        let mut input = input.as_ref().to_string();
        if input.is_empty() {
            return String::new();
        }

        let mut padding: String = String::new();

        let input_length = input.chars().count() % 3;
        if input_length > 0 {
            for _ in input_length..3 {
                padding.push('=');
                input.push('\0');
            }
        }

        let value_bytes = input.as_bytes();
        let value_iter = input.chars().enumerate().step_by(3);

        let mut result: String = String::new();

        for (i, _) in value_iter {
            if i > 0 && (i / 3 * 4) % 76 == 0 {
                result.push('\n');
            }

            let char_number = value_bytes[i];
            let char_number1 = value_bytes[i + 1];
            let char_number2 = value_bytes[i + 2];
            let n: u32 =
                ((char_number as u32) << 16) + ((char_number1 as u32) << 8) + (char_number2 as u32);

            let n1 = (n >> 18) & 0x3F;
            let n2 = (n >> 12) & 0x3F;
            let n3 = (n >> 6) & 0x3F;
            let n4 = n & 0x3F;

            let c1 = BASE64_CHARS[n1 as usize];
            let c2 = BASE64_CHARS[n2 as usize];
            let c3 = BASE64_CHARS[n3 as usize];
            let c4 = BASE64_CHARS[n4 as usize];
            result.push(c1);
            result.push(c2);
            result.push(c3);
            result.push(c4);
        }

        let padding_length = padding.chars().count();
        let result_length = result.chars().count();
        let result_length = result_length - padding_length;

        let mut result: String = result.chars().take(result_length).collect();
        result.push_str(&padding);
        result
    }

    fn decode<T: AsRef<str>>(input: T) -> Result<String, DecodeError> {
        let input = input.as_ref().to_string().replace('\n', "");
        let input_length = input.chars().count();
        if input_length % 4 != 0 {
            return Err(DecodeError::LengthError);
        }

        let base64_chars: String = BASE64_CHARS.iter().collect();
        let mut regex = "[^".to_string();
        regex.push_str(&base64_chars);
        regex.push_str("=]");
        let regex = Regex::new(&regex).unwrap();
        let input = regex.replace_all(&input, "");

        let suffix = if input.ends_with("==") {
            "AA"
        } else if input.ends_with('=') {
            "A"
        } else {
            ""
        };

        let mut input = input[..input.len() - suffix.len()].to_string();
        input.push_str(suffix);

        let value_bytes = input.as_bytes();
        let mut bytes = Vec::new();

        let input_iter = input.chars().enumerate().step_by(4);

        for (i, _) in input_iter {
            let v = Base64::index_of(value_bytes[i]);
            let v1 = Base64::index_of(value_bytes[i + 1]);
            let v2 = Base64::index_of(value_bytes[i + 2]);
            let v3 = Base64::index_of(value_bytes[i + 3]);

            let n: u32 =
                ((v as u32) << 18) + ((v1 as u32) << 12) + ((v2 as u32) << 6) + (v3 as u32);

            let c = n >> 16 & 0xFF;
            let c1 = n >> 8 & 0xFF;
            let c2 = n & 0xFF;

            bytes.push(c as u8);
            bytes.push(c1 as u8);
            bytes.push(c2 as u8);
        }

        for _ in 0..suffix.len() {
            bytes.pop();
        }

        let output = String::from_utf8(bytes).unwrap();

        Ok(output)
    }

    fn index_of(input: u8) -> usize {
        BASE64_CHARS
            .iter()
            .position(|c| c == &(input as char))
            .unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];

    let result = if input == "--decode" {
        let input = &args[2];
        Base64::decode(input)
    } else {
        Ok(Base64::encode(input))
    };

    match result {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Base64, DecodeError};

    #[test]
    fn test_empty() {
        let value = Base64::encode("");
        assert_eq!("", value);
    }

    #[test]
    fn test_f() {
        let value = Base64::encode("f");
        assert_eq!("Zg==", value);
    }

    #[test]
    fn test_fo() {
        let value = Base64::encode("fo");
        assert_eq!("Zm8=", value);
    }

    #[test]
    fn test_foo() {
        let value = Base64::encode("foo");
        assert_eq!("Zm9v", value);
    }

    #[test]
    fn test_foob() {
        let value = Base64::encode("foob");
        assert_eq!("Zm9vYg==", value);
    }

    #[test]
    fn test_fooba() {
        let value = Base64::encode("fooba");
        assert_eq!("Zm9vYmE=", value);
    }

    #[test]
    fn test_foobar() {
        let value = Base64::encode("foobar");
        assert_eq!("Zm9vYmFy", value);
    }

    #[test]
    fn test_length() {
        let result = Base64::decode("foobar").unwrap_err();
        assert_eq!(DecodeError::LengthError, result);
    }

    #[test]
    fn test_remove_newlines() {
        let result = Base64::decode("\n").unwrap();
        assert_eq!("", result);
    }

    #[test]
    fn test_regexp() {
        let result = Base64::decode("Zg{}[]==").unwrap();
        assert_eq!("f", result);
    }

    #[test]
    fn test_decode_f() {
        let value = Base64::decode("Zg==").unwrap();
        assert_eq!("f", value);
    }

    #[test]
    fn test_decode_foo() {
        let value = Base64::decode("Zm9v").unwrap();
        assert_eq!("foo", value);
    }
}
