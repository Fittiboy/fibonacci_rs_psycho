use core::ops::Add;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy)]
pub struct Thing(pub u128);

impl Add for Thing {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Thing(self.0 + other.0)
    }
}

impl Display for Thing {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

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
