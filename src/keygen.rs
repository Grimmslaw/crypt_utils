pub use std::convert::TryInto;

/// Contains fields to be used by a generator that produces keys to transform plaintext
/// using the given keyword.
#[derive(Debug)]
pub struct KeyGenerator {
    /// The word whose letters will be used as keys in a cipher
    keyword: String,
    /// Whether to retain the case of current letter (as opposed to converting all to uppercase).
    retain_case: bool,
    /// The current position of the "cursor" within the keyword
    position: i16,
}

/// A lazy generator that produces keys to use to shift plaintext to ciphertext using each letter
/// of the given keyword.
///
/// When the cursor reaches the end of the keyword, it loops back to the beginning.
impl KeyGenerator {
    /// Creates a new `KeyGenerator` with the given keyword and sets the cursor to its first letter.
    ///
    /// # Example
    ///
    /// ```
    /// let keygen = KeyGenerator::new(String::from("Queen"), true);
    /// ```
    pub fn new(keyword: String, retain_case: bool) -> KeyGenerator {
        KeyGenerator {
            keyword,
            retain_case,
            position: -1,
        }
    }
}

impl Iterator for KeyGenerator {
    type Item = char;

    /// Generates the next key from the keyword as a `char`.
    ///
    /// # Example
    ///
    /// ```
    /// let keygen = KeyGenerator::new(String::from("Queen", true));
    /// let key: char = keygen.next().unwrap();
    /// ```
    fn next(&mut self) -> Option<char> {
        if self.position as usize >= self.keyword.len() {
            self.position = 0;
        } else {
            self.position += 1;
        }

        if self.retain_case {
            // let code_point: u32 = self.keyword.chars().nth(self.position).unwrap() as u32;
            let code_point: u32 = self
                .keyword
                .chars()
                .nth(self.position.try_into().unwrap())
                .unwrap() as u32;
            println!("next codepoint: {:?}", code_point);
            Some(std::char::from_u32(code_point + 32).unwrap())
        } else {
            Some(
                self.keyword
                    .chars()
                    .nth(self.position.try_into().unwrap())
                    .unwrap(),
            )
        }
    }
}
