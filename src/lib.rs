pub struct Fib<T> {
    minus_two: T,
    minus_one: T,
}

impl<T> Iterator for Fib<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        std::mem::swap(&mut self.minus_two, &mut self.minus_one);
        self.minus_one = self.minus_two + self.minus_one;
        Some(self.minus_one)
    }
}

impl<T> Fib<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    pub fn from(first: T, second: T) -> Self {
        Fib {
            minus_two: first,
            minus_one: second,
        }
    }
}

impl Fib<u128> {
    pub fn new() -> Self {
        Fib {
            minus_two: 1,
            minus_one: 0,
        }
    }
}
