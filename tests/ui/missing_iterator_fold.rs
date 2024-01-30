#![warn(clippy::missing_iterator_fold)]

struct Countdown(u8);

impl Iterator for Countdown {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.0 = self.0.checked_sub(1)?;
        Some(self.0)
    }
}

struct Countdown2(u8);

impl Iterator for Countdown2 {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.0 = self.0.checked_sub(1)?;
        Some(self.0)
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        (0..self.0).rfold(init, f)
    }
}
