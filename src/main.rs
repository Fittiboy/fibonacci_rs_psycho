use fib::{Fib, Thing};

fn main() {
    Fib::from(Thing(1), Thing(0))
        .take(20)
        .for_each(|num| println!("{}", num));
}
