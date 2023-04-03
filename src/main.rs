use fib::{Fib, Thing};

fn main() {
    Fib::from(Thing(0), Thing(1))
        .take(20)
        .for_each(|num| println!("{}", num));
}
