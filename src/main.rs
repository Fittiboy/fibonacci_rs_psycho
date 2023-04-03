use fib::Fib;

fn main() {
    Fib::new().take(186).for_each(|num| println!("{}", num));
}
