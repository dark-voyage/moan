use crate::consts;
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

        let charsets: Vec<&[(&str, &str)]> = vec![
            consts::CHAR_MAP,
            consts::IRREGULAR_CHARS,
            consts::NUMBERS,
            consts::PUNCTUATION,
        ];
        
        for charset in charsets {
            for (l, s) in charset {
                codec.codes.push(Code::new(l.to_string(), s.to_string()));
            }
        }
        
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
