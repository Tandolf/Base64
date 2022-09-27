use regex::Regex;

use super::{DecodeError, BASE64_CHARS};

pub struct Decoder;

impl Decoder {
    pub fn decode<T: AsRef<str>>(input: T) -> Result<String, DecodeError> {
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
            let v = Decoder::index_of(value_bytes[i]);
            let v1 = Decoder::index_of(value_bytes[i + 1]);
            let v2 = Decoder::index_of(value_bytes[i + 2]);
            let v3 = Decoder::index_of(value_bytes[i + 3]);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let result = Decoder::decode("foobar").unwrap_err();
        assert_eq!(DecodeError::LengthError, result);
    }

    #[test]
    fn test_remove_newlines() {
        let result = Decoder::decode("\n").unwrap();
        assert_eq!("", result);
    }

    #[test]
    fn test_regexp() {
        let result = Decoder::decode("Zg{}[]==").unwrap();
        assert_eq!("f", result);
    }

    #[test]
    fn test_decode_f() {
        let value = Decoder::decode("Zg==").unwrap();
        assert_eq!("f", value);
    }

    #[test]
    fn test_decode_foo() {
        let value = Decoder::decode("Zm9v").unwrap();
        assert_eq!("foo", value);
    }
}
