use fib::{Fib, Thing};

fn main() {
    Fib::from(Thing(0), Thing(1))
        .take(20)
        .for_each(|num| println!("{}", num));

    Fib::new()
        .take(10)
        .for_each(|num: u128| println!("{}", num));
}
