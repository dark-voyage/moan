use super::Signal;

macro_rules! morse {
    ($($signal:tt)*) => {
        &[$(signal!($signal), )*]
    };
}
macro_rules! signal {
    (.) => {
        Signal::Dot
    };
    (-) => {
        Signal::Dash
    };
}

const ALPHABET: [&[Signal]; 26] = [
    morse!(. -),     // A
    morse!(- . . .), // B
    morse!(- . - .), // C
    morse!(- . .),   // D
    morse!(.),       // E
    morse!(. . - .), // F
    morse!(- - .),   // G
    morse!(. . . .), // H
    morse!(. .),     // I
    morse!(. - - -), // J
    morse!(- . -),   // K
    morse!(. - . .), // L
    morse!(- -),     // M
    morse!(- .),     // N
    morse!(- - -),   // O
    morse!(. - - .), // P
    morse!(- - . -), // Q
    morse!(. - .),   // R
    morse!(. . .),   // S
    morse!(-),       // T
    morse!(. . -),   // U
    morse!(. . . -), // V
    morse!(. - -),   // W
    morse!(- . . -), // X
    morse!(- . - -), // Y
    morse!(- - . .), // Z
];

const DIGITS: [&[Signal]; 10] = [
    morse!(- - - - -), // 0
    morse!(. - - - -), // 1
    morse!(. . - - -), // 2
    morse!(. . . - -), // 3
    morse!(. . . . -), // 4
    morse!(. . . . .), // 5
    morse!(- . . . .), // 6
    morse!(- - . . .), // 7
    morse!(- - - . .), // 8
    morse!(- - - - .), // 9
];

macro_rules! punctuation {
    ($name:ident, $($signal:tt)*) => {
        const $name: &'static [Signal] = morse!($($signal)*);
    };
}

punctuation!(PERIOD,        . - . - . -);
punctuation!(COMMA,         - - . . - -);
punctuation!(QUESTION,      . . - - . .);
punctuation!(APOSTROPHE,    . - - - - .);
punctuation!(EXCLAMATION,   - . - . - -);
punctuation!(SLASH,         - . . - .);
punctuation!(OPEN_PAREN,    - . - - .);
punctuation!(CLOSE_PAREN,   - . - - . -);
punctuation!(AMPERSAND,     . - . . .);
punctuation!(COLON,         - - - . . .);
punctuation!(SEMICOLON,     - . - . - .);
punctuation!(EQUALS,        - . . . -);
punctuation!(PLUS,          . - . - .);
punctuation!(MINUS,         - . . . . -);
punctuation!(UNDERSCORE,    . . - - . -);
punctuation!(QUOTE,         . - . . - .);
punctuation!(DOLLAR,        . . . - . . -);
punctuation!(AT,            . - - . - .);

pub(crate) fn get_signals(ch: char) -> Option<&'static [Signal]> {
    match ch {
        '0'..='9' => Some(DIGITS[(ch as u8 - b'0') as usize]),
        'A'..='Z' => Some(ALPHABET[(ch as u8 - b'A') as usize]),
        'a'..='z' => Some(ALPHABET[(ch as u8 - b'a') as usize]),
        '.' => Some(PERIOD),
        ',' => Some(COMMA),
        '?' => Some(QUESTION),
        '\'' => Some(APOSTROPHE),
        '!' => Some(EXCLAMATION),
        '/' => Some(SLASH),
        '(' => Some(OPEN_PAREN),
        ')' => Some(CLOSE_PAREN),
        '&' => Some(AMPERSAND),
        ':' => Some(COLON),
        ';' => Some(SEMICOLON),
        '=' => Some(EQUALS),
        '+' => Some(PLUS),
        '-' => Some(MINUS),
        '_' => Some(UNDERSCORE),
        '"' => Some(QUOTE),
        '$' => Some(DOLLAR),
        '@' => Some(AT),
        _ => None,
    }
}

pub(crate) fn get_signals_with_fallback(ch: char) -> &'static [Signal] {
    get_signals(ch).unwrap_or(QUESTION)
}