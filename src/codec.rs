use std::fmt::Display;
use crate::codes::Code;
pub struct Codec {
    head: Code,
    codes: Vec<Code>,
}

impl Display for Codec {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = String::new();
        for i in 0..self.codes.len() {
            s.push_str(self.codes[i].to_string().as_ref());
            s.push_str("\n");
        }
    write!(f, "{s}")
    }
}

impl Codec {
    pub fn retreave_head(&self) -> Code {
        return self.head.clone();
    }
    pub fn new() -> Codec {
        let head = Code::new(String::from(""), String::from(""));
        let mut codec = Codec {
            head,
            codes: Vec::new(),
        };

        codec
            .codes
            .push(Code::new(String::from("A"), String::from(".-")));
        codec
            .codes
            .push(Code::new(String::from("B"), String::from("-...")));
        codec
            .codes
            .push(Code::new(String::from("C"), String::from("-.-.")));
        codec
            .codes
            .push(Code::new(String::from("D"), String::from("-..")));
        codec
            .codes
            .push(Code::new(String::from("E"), String::from(".")));
        codec
            .codes
            .push(Code::new(String::from("F"), String::from("..-.")));

        codec
            .codes
            .push(Code::new(String::from("G"), String::from("--.")));
        codec
            .codes
            .push(Code::new(String::from("H"), String::from("....")));

        codec
            .codes
            .push(Code::new(String::from("I"), String::from("..")));

        codec
            .codes
            .push(Code::new(String::from("J"), String::from(".---")));

        codec
            .codes
            .push(Code::new(String::from("K"), String::from("-.-")));

        codec
            .codes
            .push(Code::new(String::from("L"), String::from(".-..")));

        codec
            .codes
            .push(Code::new(String::from("M"), String::from("--")));

        codec
            .codes
            .push(Code::new(String::from("N"), String::from("-.")));

        codec
            .codes
            .push(Code::new(String::from("O"), String::from("---")));

        codec
            .codes
            .push(Code::new(String::from("P"), String::from(".--.")));

        codec
            .codes
            .push(Code::new(String::from("Q"), String::from("--.-")));

        codec
            .codes
            .push(Code::new(String::from("R"), String::from(".-.")));

        codec
            .codes
            .push(Code::new(String::from("S"), String::from("...")));

        codec
            .codes
            .push(Code::new(String::from("T"), String::from("-")));

        codec
            .codes
            .push(Code::new(String::from("U"), String::from("..-")));

        codec
            .codes
            .push(Code::new(String::from("V"), String::from("...-")));

        codec
            .codes
            .push(Code::new(String::from("W"), String::from(".--")));

        codec
            .codes
            .push(Code::new(String::from("X"), String::from("-..-")));

        codec
            .codes
            .push(Code::new(String::from("Y"), String::from("-.--")));
        codec
            .codes
            .push(Code::new(String::from("Z"), String::from("--..")));
        codec
            .codes
            .push(Code::new(String::from("Ä"), String::from(".-.-")));
        codec
            .codes
            .push(Code::new(String::from("Ö"), String::from("---.")));
        codec
            .codes
            .push(Code::new(String::from("Ü"), String::from("..--")));
        codec
            .codes
            .push(Code::new(String::from("CH"), String::from("----")));
        codec
            .codes
            .push(Code::new(String::from("0"), String::from("-----")));
        codec
            .codes
            .push(Code::new(String::from("1"), String::from(".----")));
        codec
            .codes
            .push(Code::new(String::from("2"), String::from("..---")));
        codec
            .codes
            .push(Code::new(String::from("3"), String::from("...--")));
        codec
            .codes
            .push(Code::new(String::from("4"), String::from("....-")));
        codec
            .codes
            .push(Code::new(String::from("5"), String::from(".....")));
        codec
            .codes
            .push(Code::new(String::from("6"), String::from("-....")));
        codec
            .codes
            .push(Code::new(String::from("7"), String::from("--...")));
        codec
            .codes
            .push(Code::new(String::from("8"), String::from("---..")));
        codec
            .codes
            .push(Code::new(String::from("9"), String::from("----.")));
        codec
            .codes
            .push(Code::new(String::from("."), String::from(".-.-.-")));
        codec
            .codes
            .push(Code::new(String::from(","), String::from("--..--")));
        codec
            .codes
            .push(Code::new(String::from("?"), String::from("..--..")));
        codec
            .codes
            .push(Code::new(String::from("!"), String::from("..--.")));
        codec
            .codes
            .push(Code::new(String::from(":"), String::from("---...")));
        codec
            .codes
            .push(Code::new(String::from("\""), String::from(".-..-.")));
        codec
            .codes
            .push(Code::new(String::from("\'"), String::from(".----.")));
        codec
            .codes
            .push(Code::new(String::from("="), String::from("-...-")));
        for i in 0..codec.codes.len() {
            for j in 0..codec.codes.len() {
                if codec.codes[i].get_lenght_of_seq() < codec.codes[j].get_lenght_of_seq() {
                    let temp = codec.codes[i].clone();
                    codec.codes[i] = codec.codes[j].clone();
                    codec.codes[j] = temp;
                }
            }
        }
        for i in 0..codec.codes.len() {
            codec.head.insert_node(codec.codes[i].clone());
        }
        return codec;
    }
}