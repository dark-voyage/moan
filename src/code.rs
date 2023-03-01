use std::fmt::Display;

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
pub struct Cod {
    letter: String,
    sequence: String,
}

impl Display for Cod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Letter: {} | Sequence: {}",
            self.get_letter(),
            self.get_sequence()
        )
    }
}

impl Cod {
    pub fn new(l: String, s: String) -> Cod {
        return Cod {
            letter: l,
            sequence: s,
        };
    }
    pub fn get_letter(&self) -> String {
        return self.letter.to_string();
    }
    pub fn get_sequence(&self) -> String {
        return self.sequence.to_string();
    }
}
