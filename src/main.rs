use fib::Fib;

fn main() {
    Fib::new().take(20).for_each(|num| println!("{}", num));
}
