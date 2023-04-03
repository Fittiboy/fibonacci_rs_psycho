pub struct Fib {
    minus_two: u128,
    minus_one: u128,
}

impl Iterator for Fib {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        std::mem::swap(&mut self.minus_two, &mut self.minus_one);
        self.minus_one = self.minus_two + self.minus_one;
        Some(self.minus_one)
    }
}

impl Fib {
    pub fn new() -> Self {
        Fib {
            minus_two: 1,
            minus_one: 0,
        }
    }
}
