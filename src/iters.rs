use super::Signal;

/// Within an [`Action`], whether it is to turn the signalling mechanism on or off
///
/// [`Action`]: struct.Action.html
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum State {
    /// The action is to turn it on
    On,
    /// The action is to turn it off
    Off,
}

/// `DelayType` indicates whether a duration should be slowed down when using the Farnsworth method
/// of learning Morse code.
///
/// When learning Morse code, it is common to slow down the delays between characters and words,
/// while keeping the speed within an individual character fast.  This is known as the "Farnsworth"
/// method for learning morse code.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DelayType {
    /// The delay should not be slowed down to the slower Farnsworth speed
    Regular,
    /// The delay should be slowed down to the generally slower Farnsworth speed
    Farnsworth,
}

/// An `Action` is how the library tells you what to signal.
///
/// In general, this library affords lots of flexibility in how the morse code is transmitted by
/// leaving it entirely up to the user.  Instead of doing it for you, it supplies an iterator of
/// actions that need to be done.
///
/// In order to send morse code, you need to turn a signal on and off for different lengths of
/// time.  An `Action`, correspondingly, has two parts: the `state` (whether it is on or off) and
/// the `duration`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Action {
    /// The `duration` of an action is measured in units of one dot-length.  A dot-length is
    /// typically around 50 to 100 milliseconds or so.
    pub duration: u8,
    /// Whether the duration should use the regular speed or the Farnsworth speed
    pub delay_type: DelayType,
    /// Whether it should be on or off
    pub state: State,
}

impl Action {
    fn word_break() -> Self {
        Action {
            duration: 7,
            delay_type: DelayType::Farnsworth,
            state: State::Off,
        }
    }
    fn char_break() -> Self {
        Action {
            duration: 3,
            delay_type: DelayType::Farnsworth,
            state: State::Off,
        }
    }
    fn signal_break() -> Self {
        Action {
            duration: 1,
            delay_type: DelayType::Regular,
            state: State::Off,
        }
    }
}
impl From<Signal> for Action {
    fn from(signal: Signal) -> Action {
        let duration = match signal {
            Signal::Dot => 1,
            Signal::Dash => 3,
        };
        Action {
            duration,
            delay_type: DelayType::Regular,
            state: State::On,
        }
    }
}

/// A `MorseIter` is an iterator that yields [`Action`]s. Create a `MorseIter` with either the
/// `MorseIter::new` method or the `iter` function.
///
/// [`Action`]: struct.Action.html
#[derive(Debug)]
pub struct Iters<'a> {
    words: WordsIter<'a>,
    current_word: Option<Word<'a>>,
}
impl<'a> Iters<'a> {
    /// Creates a new `MorseIter` that encodes the given text
    pub fn new(text: &'a str) -> Self {
        let mut words = WordsIter::new(text);
        let first_word = words.next().map(Word::new);
        Iters {
            words,
            current_word: first_word,
        }
    }
}

impl<'a> Iterator for Iters<'a> {
    type Item = Action;

    fn next(&mut self) -> Option<Action> {
        if let Some(ref mut current_word) = self.current_word {
            let res = current_word.next();
            if res.is_some() {
                return res;
            }
        }

        if let Some(new_word) = self.words.next() {
            self.current_word = Some(Word::new(new_word));
            Some(Action::word_break())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Word<'a> {
    chars: core::str::Chars<'a>,
    current_char: Option<Char>,
}
impl<'a> Word<'a> {
    fn new(text: &'a str) -> Self {
        let mut chars = text.chars();
        let first_char = chars.next().map(Char::new);
        Word {
            chars,
            current_char: first_char,
        }
    }
}
impl<'a> Iterator for Word<'a> {
    type Item = Action;

    fn next(&mut self) -> Option<Action> {
        if let Some(ref mut current_char) = self.current_char {
            let res = current_char.next();
            if res.is_some() {
                return res;
            }
        }

        if let Some(new_char) = self.chars.next() {
            self.current_char = Some(Char::new(new_char));
            Some(Action::char_break())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Char {
    signals: core::slice::Iter<'static, Signal>,
    need_space: bool,
}
impl Char {
    fn new(ch: char) -> Self {
        Char {
            signals: super::chars::get_signals_with_fallback(ch).iter(),
            need_space: false,
        }
    }
}
impl Iterator for Char {
    type Item = Action;

    fn next(&mut self) -> Option<Action> {
        if self.need_space {
            self.need_space = false;
            Some(Action::signal_break())
        } else if let Some(&next_signal) = self.signals.next() {
            if !self.signals.as_slice().is_empty() {
                self.need_space = true;
            }
            Some(next_signal.into())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct WordsIter<'a> {
    current_str: &'a str,
}
impl<'a> WordsIter<'a> {
    fn new(text: &'a str) -> Self {
        WordsIter { current_str: text }
    }
}
impl<'a> Iterator for WordsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.current_str = self.current_str.trim_start();
        if self.current_str.is_empty() {
            return None;
        }

        let (alphanum_word, rest) = {
            if let Some(index) = self.current_str.find(|c: char| !c.is_alphanumeric()) {
                self.current_str.split_at(index)
            } else {
                (self.current_str, "")
            }
        };

        if alphanum_word.is_empty() {
            // Yield the first non-alphanumeric, non-whitespace character
            let (head, tail) = {
                let new_str = rest.trim_start();
                if new_str.is_empty() {
                    return None;
                }
                let index = new_str
                    .char_indices()
                    .nth(1)
                    .map_or(new_str.len(), |(idx, _)| idx);
                new_str.split_at(index)
            };
            self.current_str = tail;
            Some(head)
        } else {
            self.current_str = rest;
            Some(alphanum_word)
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use super::*;
    use crate::Signal::*;

    #[test]
    fn words_test() {
        let sentence = "The quick brown fox???oh no123 45!";
        let words = WordsIter::new(sentence).collect::<Vec<_>>();
        assert_eq!(
            words,
            &["The", "quick", "brown", "fox", "?", "?", "?", "oh", "no123", "45", "!"]
        );
    }

    #[test]
    fn char_test_e() {
        let ch = Char::new('e');
        let actions = ch.collect::<Vec<_>>();
        assert_eq!(actions, &[Dot.into()]);
    }
    #[test]
    fn char_test_a() {
        let ch = Char::new('A');
        let actions = ch.collect::<Vec<_>>();
        assert_eq!(actions, &[Dot.into(), Action::signal_break(), Dash.into()]);
    }
}