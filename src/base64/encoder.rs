use super::BASE64_CHARS;

pub struct Encoder {
    cols: usize,
}

impl Default for Encoder {
    fn default() -> Self {
        Self { cols: 76 }
    }
}

impl Encoder {
    pub fn new(cols: usize) -> Encoder {
        Encoder { cols }
    }

    pub fn encode<T: AsRef<str>>(&self, input: T) -> String {
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
            if i > 0 && (i / 3 * 4) % self.cols == 0 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let encoder = Encoder::default();
        let value = encoder.encode("");
        assert_eq!("", value);
    }

    #[test]
    fn test_f() {
        let encoder = Encoder::default();
        let value = encoder.encode("f");
        assert_eq!("Zg==", value);
    }

    #[test]
    fn test_fo() {
        let encoder = Encoder::default();
        let value = encoder.encode("fo");
        assert_eq!("Zm8=", value);
    }

    #[test]
    fn test_foo() {
        let encoder = Encoder::default();
        let value = encoder.encode("foo");
        assert_eq!("Zm9v", value);
    }

    #[test]
    fn test_foob() {
        let encoder = Encoder::default();
        let value = encoder.encode("foob");
        assert_eq!("Zm9vYg==", value);
    }

    #[test]
    fn test_fooba() {
        let encoder = Encoder::default();
        let value = encoder.encode("fooba");
        assert_eq!("Zm9vYmE=", value);
    }

    #[test]
    fn test_foobar() {
        let encoder = Encoder::default();
        let value = encoder.encode("foobar");
        assert_eq!("Zm9vYmFy", value);
    }
}
