use std::str::Chars;

pub(crate) const EOF_CHAR: char = '\0';

pub struct Cursor<'i> {
    pub initial_len: usize,
    chars: Chars<'i>,
}

impl<'i> Cursor<'i> {
    pub fn new(s: &'i str) -> Cursor {
        Cursor {
            initial_len: s.len(),
            chars: s.chars(),
        }
    }

    /// Peeks the next symbol from the input stream without consuming it.
    pub fn first(&self) -> char {
        self.nth_char(0)
    }

    /// Peeks the second symbol from the input stream without consuming it.
    pub fn second(&self) -> char {
        self.nth_char(1)
    }

    /// Returns amount of already consumed symbols.
    pub fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    /// Moves to the next character.
    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        Some(c)
        //
        // #[cfg(debug_assertions)]
        // {
        //     self.prev = c;
        // }
    }

    pub fn bump_n_times(&mut self, n: usize) {
        for _ in 0..n {
            self.bump();
        }
    }

    /// Checks if there is nothing more to consume.
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    /// Returns amount of eaten symbols.
    pub fn consume_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let mut consumed: usize = 0;
        while predicate(self.first()) && !self.is_eof() {
            consumed += 1;
            self.bump();
        }

        consumed
    }

    /// Returns a `Chars` iterator over the remaining characters.
    pub fn chars(&self) -> Chars<'i> {
        self.chars.clone()
    }

    pub fn is_next(&self, s: &str) -> bool {
        s.chars().enumerate().all(|(i, c)| self.nth_char(i) == c)
    }

    pub fn consume_if_next(&mut self, s: &str) -> bool {
        if self.is_next(s) {
            self.bump_n_times(s.len());
            true
        } else {
            false
        }
    }

    /// Returns nth character relative to the current cursor position.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF_CHAR)
    }
}
