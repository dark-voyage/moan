#![allow(dead_code)]

pub const CHAR_MAP: &[(&str, &str)] = &[
  ("A", "Ah"),
  ("B", "hAAA"),
  ("C", "hAhA"),
  ("D", "hAA"),
  ("E", "A"),
  ("F", "AAhA"),
  ("G", "hhA"),
  ("H", "AAAA"),
  ("I", "AA"),
  ("J", "Ahhh"),
  ("K", "hAh"),
  ("L", "AhAA"),
  ("M", "hh"),
  ("N", "hA"),
  ("O", "hhh"),
  ("P", "AhhA"),
  ("Q", "hhAh"),
  ("R", "AhA"),
  ("S", "AAA"),
  ("T", "h"),
  ("U", "AAh"),
  ("V", "AAAh"),
  ("W", "Ahh"),
  ("X", "hAAh"),
  ("Y", "hAhh"),
  ("Z", "hhAA"),
];

pub const IRREGULAR_CHARS : &[(&str, &str)] = &[
  ("Ä", "AhAh"),
  ("Ö", "hhhA"),
  ("Ü", "AAhh"),
  ("CH", "hhhh"),
];

pub const NUMBERS: &[(&str, &str)] = &[
  ("0", "hhhhh"),
  ("1", "Ahhhh"),
  ("2", "AAhhh"),
  ("3", "AAAhh"),
  ("4", "AAAAh"),
  ("5", "AAAAA"),
  ("6", "hAAAA"),
  ("7", "hhAAA"),
  ("8", "hhhAA"),
  ("9", "hhhhA"),
];

pub const PUNCTUATION: &[(&str, &str)] = &[
  (".", "AhAhAh"),
  (",", "hhAAhh"),
  ("?", "AAhhAA"),
  ("!", "AAhhA"),
  (":", "hhhAAA"),
  ("\"", "AhAAhA"),
  ("\'", "AhhhhA"),
  ("=", "hAAAh"),
];


