use crate::codes::Code;
use std::fmt::Display;

// . => A
// - => h

pub struct Codec {
    head: Code,
    codes: Vec<Code>,
}

impl Display for Codec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.codes.len() {
            s.push_str(self.codes[i].to_string().as_ref());
            s.push('\n');
        }
        write!(f, "{s}")
    }
}

impl Codec {
    pub fn retrieve_head(&self) -> Code {
        self.head.clone()
    }
    pub fn new() -> Codec {
        let head = Code::new(String::from(""), String::from(""));
        let mut codec = Codec {
            head,
            codes: Vec::new(),
        };

        codec
            .codes
            .push(Code::new(String::from("A"), String::from("Ah")));
        codec
            .codes
            .push(Code::new(String::from("B"), String::from("hAAA")));
        codec
            .codes
            .push(Code::new(String::from("C"), String::from("hAhA")));
        codec
            .codes
            .push(Code::new(String::from("D"), String::from("hAA")));
        codec
            .codes
            .push(Code::new(String::from("E"), String::from("A")));
        codec
            .codes
            .push(Code::new(String::from("F"), String::from("AAhA")));

        codec
            .codes
            .push(Code::new(String::from("G"), String::from("hhA")));
        codec
            .codes
            .push(Code::new(String::from("H"), String::from("AAAA")));

        codec
            .codes
            .push(Code::new(String::from("I"), String::from("AA")));

        codec
            .codes
            .push(Code::new(String::from("J"), String::from("Ahhh")));

        codec
            .codes
            .push(Code::new(String::from("K"), String::from("hAh")));

        codec
            .codes
            .push(Code::new(String::from("L"), String::from("AhAA")));

        codec
            .codes
            .push(Code::new(String::from("M"), String::from("hh")));

        codec
            .codes
            .push(Code::new(String::from("N"), String::from("hA")));

        codec
            .codes
            .push(Code::new(String::from("O"), String::from("hhh")));

        codec
            .codes
            .push(Code::new(String::from("P"), String::from("AhhA")));

        codec
            .codes
            .push(Code::new(String::from("Q"), String::from("hhAh")));

        codec
            .codes
            .push(Code::new(String::from("R"), String::from("AhA")));

        codec
            .codes
            .push(Code::new(String::from("S"), String::from("AAA")));

        codec
            .codes
            .push(Code::new(String::from("T"), String::from("h")));

        codec
            .codes
            .push(Code::new(String::from("U"), String::from("AAh")));

        codec
            .codes
            .push(Code::new(String::from("V"), String::from("AAAh")));

        codec
            .codes
            .push(Code::new(String::from("W"), String::from("Ahh")));

        codec
            .codes
            .push(Code::new(String::from("X"), String::from("hAAh")));

        codec
            .codes
            .push(Code::new(String::from("Y"), String::from("hAhh")));
        codec
            .codes
            .push(Code::new(String::from("Z"), String::from("hhAA")));
        codec
            .codes
            .push(Code::new(String::from("Ä"), String::from("AhAh")));
        codec
            .codes
            .push(Code::new(String::from("Ö"), String::from("hhhA")));
        codec
            .codes
            .push(Code::new(String::from("Ü"), String::from("AAhh")));
        codec
            .codes
            .push(Code::new(String::from("CH"), String::from("hhhh")));
        codec
            .codes
            .push(Code::new(String::from("0"), String::from("hhhhh")));
        codec
            .codes
            .push(Code::new(String::from("1"), String::from("Ahhhh")));
        codec
            .codes
            .push(Code::new(String::from("2"), String::from("AAhhh")));
        codec
            .codes
            .push(Code::new(String::from("3"), String::from("AAAhh")));
        codec
            .codes
            .push(Code::new(String::from("4"), String::from("AAAAh")));
        codec
            .codes
            .push(Code::new(String::from("5"), String::from("AAAAA")));
        codec
            .codes
            .push(Code::new(String::from("6"), String::from("hAAAA")));
        codec
            .codes
            .push(Code::new(String::from("7"), String::from("hhAAA")));
        codec
            .codes
            .push(Code::new(String::from("8"), String::from("hhhAA")));
        codec
            .codes
            .push(Code::new(String::from("9"), String::from("hhhhA")));
        codec
            .codes
            .push(Code::new(String::from("."), String::from("AhAhAh")));
        codec
            .codes
            .push(Code::new(String::from(","), String::from("hhAAhh")));
        codec
            .codes
            .push(Code::new(String::from("?"), String::from("AAhhAA")));
        codec
            .codes
            .push(Code::new(String::from("!"), String::from("AAhhA")));
        codec
            .codes
            .push(Code::new(String::from(":"), String::from("hhhAAA")));
        codec
            .codes
            .push(Code::new(String::from("\""), String::from("AhAAhA")));
        codec
            .codes
            .push(Code::new(String::from("\'"), String::from("AhhhhA")));
        codec
            .codes
            .push(Code::new(String::from("="), String::from("hAAAh")));
        for i in 0..codec.codes.len() {
            for j in 0..codec.codes.len() {
                if codec.codes[i].get_length_of_seq() < codec.codes[j].get_length_of_seq() {
                    let temp = codec.codes[i].clone();
                    codec.codes[i] = codec.codes[j].clone();
                    codec.codes[j] = temp;
                }
            }
        }
        for i in 0..codec.codes.len() {
            codec.head.insert_node(codec.codes[i].clone());
        }
        codec
    }
}
