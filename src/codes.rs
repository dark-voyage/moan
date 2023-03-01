use crate::code::Cod;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]

pub struct Code {
    letter: String,
    sequence: String,
    depth: usize,
    left_code: Option<Box<Code>>,
    right_code: Option<Box<Code>>,
}

// impl Display for Code {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

impl Code {
    pub fn new(letter: String, sequence: String) -> Code {
        Code {
            letter,
            sequence,
            depth: 0,
            left_code: None,
            right_code: None,
        }
    }

    pub fn get_length_of_seq(&self) -> i32 {
        self.sequence.len() as i32
    }
    pub fn get_sequence(&self) -> String {
        self.sequence.to_string()
    }

    pub fn get_letter(&self) -> String {
        self.letter.to_string()
    }

    pub fn to_string(&self) -> String {
        let left = match &self.left_code {
            Some(c) => c.get_letter(),
            None => String::from("None"),
        };
        let right = match &self.right_code {
            Some(c) => c.get_letter(),
            None => String::from("None"),
        };
        format!(
            "Letter: {} | Sequence: {} | depth: {} | left: {} | right: {}",
            self.letter, self.sequence, self.depth, left, right
        )
    }

    pub fn find_letter_for_sequence(&self, mut sequence: String) -> String {
        if sequence.is_empty() {
            return self.letter.to_string();
        }
        let character: char = sequence.remove(0);
        if character == '.' {
            return search_in_node(sequence, self.left_code.clone());
        }
        search_in_node(sequence, self.right_code.clone())
    }

    pub fn get_children(&self, mut stack: HashSet<Cod>) -> HashSet<Cod> {
        let _t = match &self.left_code {
            Some(x) => {
                for codec_element in x.get_children(stack.clone()) {
                    stack.insert(Cod::new(
                        codec_element.get_letter(),
                        codec_element.get_sequence(),
                    ));
                }
                let code = self.clone();
                stack.insert(Cod::new(code.get_letter(), code.get_sequence()));
                true
            }
            None => true,
        };
        let _m = match &self.right_code {
            Some(x) => {
                for codec_element in x.get_children(stack.clone()) {
                    stack.insert(Cod::new(
                        codec_element.get_letter(),
                        codec_element.get_sequence(),
                    ));
                }
                let code = self.clone();
                stack.insert(Cod::new(code.get_letter(), code.get_sequence()));
                true
            }
            None => true,
        };
        stack
    }

    fn insert_on_right(&mut self, code: Code) {
        match self.right_code.as_mut() {
            Some(c) => c.insert_node(code),
            None => self.right_code = Option::from(Box::new(code)),
        }
    }

    fn insert_on_left(&mut self, code: Code) {
        match self.left_code.as_mut() {
            Some(c) => c.insert_node(code),

            None => self.left_code = Option::from(Box::new(code)),
        }
    }

    pub fn insert_node(&mut self, mut code: Code) {
        let current_character = code.seq_at(code.depth);
        code.increment_depth();
        if current_character == "." {
            self.insert_on_left(code);
        } else {
            self.insert_on_right(code);
        }
    }

    pub fn seq_at(&self, index: usize) -> String {
        assert!(self.get_length_of_seq() > index as i32);
        return (self.sequence.as_bytes()[index] as char).to_string();
    }

    pub fn increment_depth(&mut self) {
        self.depth += 1;
    }
}
pub fn search_in_node(sequence: String, node: Option<Box<Code>>) -> String {
    match &node {
        Some(c) => c.find_letter_for_sequence(sequence),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flattening() {
        let mut codes = Code::new(String::from(""), String::from(""));
        codes.insert_node(Code::new(String::from("E"), String::from(".")));
        let stack = codes.get_children(HashSet::new());
        assert_eq!(stack.len(), 1);
        
        for code in stack {
            assert_eq!(code.to_string(), "Letter:  | Sequence: ");
        }
    }
}