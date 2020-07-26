//! Utilities for use with classical cryptographic functions (e.g. Caesar, Vigenere, etc.)
//!
//! DISCLAIMER: This module and its functions are purely for entertainment or study. It is
//! in no way appropriate for any situation in which cryptographic security is required.

/// Structs and functions specifically having to do with processes that make classical
/// cryptography possible.
pub mod crypt {
    use std::convert::TryInto;

    const DEFAULT_CHAR_OFFSET: u32 = 65;
    const ALPHABET_SIZE: u32 = 26;

    /// Contains fields to be used by a generator that produces keys to transform plaintext
    /// using the given keyword.
    struct KeyGenerator {
        /// The word whose letters will be used as keys in a cipher
        keyword: String,
        /// Whether to retain the case of current letter (as opposed to converting all to uppercase).
        retain_case: bool,
        /// The current position of the "cursor" within the keyword
        position: u32,
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
        /// let keygen = KeyGenerator::new(String::from("Queen", true));
        /// ```
        pub fn new(keyword: String, retain_case: bool) -> KeyGenerator {
            KeyGenerator {
                keyword,
                retain_case,
                position: 0,
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

    /// Shifts the given `char` by the given amount, wrapping back around to the beginning of
    /// the alphabet if `shift_by` is greater than the amount of letters left in the alphabet.
    ///
    /// # Example
    ///
    /// ```
    /// let shifted: char = KeyGenerator::wrapped_shift_letters('a', 12u32);
    /// ```
    pub fn wrapped_shift_letters(to_shift: char, shift_by: u32) -> char {
        let code_point = to_shift as u32;
        if (65..91).contains(&code_point) || (97..123).contains(&code_point) {
            let transformed = (((code_point - DEFAULT_CHAR_OFFSET) + shift_by) % ALPHABET_SIZE)
                + DEFAULT_CHAR_OFFSET;
            std::char::from_u32(transformed).unwrap_or(to_shift)
        } else {
            to_shift
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
