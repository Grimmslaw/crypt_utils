//! Utilities for use with classical cryptographic functions (e.g. Caesar, Vigenere, etc.)
//!
//! DISCLAIMER: This module and its functions are purely for entertainment or study. It is
//! in no way appropriate for any situation in which cryptographic security is required.

mod keygen;
pub use keygen::KeyGenerator;

/// The default offset for uppercase letters (which equates to the ASCII code point of the letter 'A')
const UPPERCASE_OFFSET: u32 = 65;
/// The default offset for lowercase letters (which equates to the ASCII code point of the letter 'a')
const LOWERCASE_OFFSET: u32 = 97;
/// The default size of the an alphabet (in this case A-Z or a-z)
const ALPHABET_SIZE: u32 = 26;

fn normalize_char_codepoint(start: char) -> Option<(u32, u32)> {
    let start_code_point = start as u32;
    let offset_to_use: u32 = if (65..91).contains(&start_code_point) {
        UPPERCASE_OFFSET
    } else if (97..123).contains(&start_code_point) {
        LOWERCASE_OFFSET
    } else {
        0u32
    };

    if offset_to_use != 0u32 {
        let normalized_char_point = (start_code_point - offset_to_use) % ALPHABET_SIZE;
        println!("normalized_char_point = {:?}", normalized_char_point);
        Some((normalized_char_point, offset_to_use))
    } else {
        None
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
pub fn wrapped_shift_letters(to_shift: char, key_char: char) -> char {
    let normalized_start = normalize_char_codepoint(to_shift);
    println!("normalized_start = {:?}", normalized_start);
    let normalized_shift = normalize_char_codepoint(key_char);
    println!("normalized_shift = {:?}", normalized_shift);
    if normalized_start == None || normalized_shift == None {
        println!(
            "Either normalized_start ({:?}) or normalized_shift ({:?}) is `None`",
            normalized_start, normalized_shift
        );
        return to_shift;
    }

    let (start, offset) = normalized_start.unwrap();
    let (shift, _) = normalized_shift.unwrap();

    let transformed: u32 = start + shift + offset;
    std::char::from_u32(transformed).unwrap_or(to_shift)
}

#[cfg(test)]
mod tests {
    use super::keygen::KeyGenerator;
    use super::wrapped_shift_letters;
    #[test]
    fn shifts_correct_amount() {
        let mut keygen = KeyGenerator::new(String::from("queen"), false);
        let key = keygen.next().unwrap();
        println!("key to shift by: {:?}", key);

        let shifted = wrapped_shift_letters('a', key);
        assert_eq!(shifted, 'q');
    }
}
