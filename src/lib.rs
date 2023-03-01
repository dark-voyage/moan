//#![deny(missing_docs)]
mod code;
mod codec;
mod codes;
/// decoder used to decode the morse code
pub mod decoder;
/// counterpart to the decoder, encoder is encoding the letters into morse code
pub mod encoder;
#[cfg(test)]
mod tests {
    use crate::decoder::Decoder;
    use crate::encoder::Encoder;

    #[test]
    fn decode_messages() {
        let dec_message = ".";
        let decoder = Decoder::new();
        assert_eq!(decoder.decode_message(dec_message.to_string()), "E");
    }

    #[test]
    fn encode_message() {
        let letter_message = "HELLO";
        let encoder = Encoder::new();
        assert_eq!(
            encoder.encode_letters(letter_message.to_string()),
            String::from(".... . .-.. .-.. ---")
        );
    }

    #[test]
    fn encode_and_decode() {
        let letter_message = "HELLO";
        let encoder = Encoder::new();
        let decoder = Decoder::new();
        let encoded_message = encoder.encode_letters(letter_message.to_string());
        assert_eq!(decoder.decode_message(encoded_message), "HELLO");
    }
}
