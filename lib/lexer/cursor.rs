use std::str::Chars;

pub struct Cursor<'a> {
    chars: Chars<'a>
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars()
        }
    }

    pub fn peek_first(&self) -> Option<char> {
        self.chars.clone().next()
    }
    #[allow(unused)]
    pub fn peak_second(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next()
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }
}