use fib::{Fib, Thing};

fn main() {
    Fib::new()
        .take(10)
        .for_each(|num: Thing| println!("{}", num));
}
