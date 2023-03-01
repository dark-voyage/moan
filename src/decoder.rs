#![deny(missing_docs)]
use crate::codec::Codec;
/// A Structure holding the morse codec, in order to decode a morse sequence into a letter
/// that is listed in the international morse code.
pub struct Decoder {
    codec: Codec,
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder {
    /// Create a new Decoder.
    /// Will create a new Codec during the creation of the Decoder.
    pub fn new() -> Decoder {
        Decoder {
            codec: Codec::new(),
        }
    }
    /// For a given message, iterate over the space seperated message parts in the morse code.
    /// Search the letter that is represented by the sequenece of {.} and {-} given in as the
    /// morse_sequence parameter.
    pub fn decode_message(&self, morse_sequence: String) -> String {
        let chunks: Vec<_> = morse_sequence.split_whitespace().collect();
        let mut message: String = String::new();
        for sequence in chunks {
            message.push_str(
                &self
                    .codec
                    .retrieve_head()
                    .find_letter_for_sequence(sequence.to_string()),
            );
        }
        message
    }
}