#![deny(missing_docs)]
use crate::code::Cod;
use crate::codec::Codec;
use std::collections::HashSet;
/// Structure for the encoding of a lettered string into a morse sequence.
/// Is creating a new instance of the codec, when created.
/// Should be reused.
pub struct Encoder {
    codec: Codec,
    stack: HashSet<Cod>,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    /// init a new encoder, that is creating a new codec, for the morse coder.
    pub fn new() -> Encoder {
        let mut encoder = Encoder {
            codec: Codec::new(),
            stack: HashSet::new(),
        };
        encoder.stack = encoder.codec.retrieve_head().get_children(encoder.stack);
        encoder
    }
    /// encode a lettered message into a sequence of morse code
    /// # Example
    /// ```
    /// use moan::encoder;
    /// let encoder = encoder::Encoder::new();
    /// let message = encoder.encode_letters(String::from("ET"));
    /// println!("message ==> {}", message);
    /// // message ==> ". -"
    /// ```
    pub fn encode_letters(&self, letter_sequence: String) -> String {
        let letters: Vec<_> = letter_sequence.split("").collect();
        let mut message: String = String::new();
        for letter_el in letters {
            if letter_el.is_empty() {
                continue;
            }
            for code in &self.stack {
                if code.get_letter() == letter_el {
                    message.push_str(&code.get_sequence());
                    message.push(' ');
                    continue;
                }
            }
        }
        message.pop();
        message
    }
}