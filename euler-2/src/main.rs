struct Fib {
    prev: u64,
    curr: u64,
}

impl Fib {
    fn new() -> Fib {
        Fib { curr: 1, prev: 1 }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let temp = self.curr + self.prev;
        self.prev = self.curr;
        self.curr = temp;
        Some(self.prev)
    }
}


fn main() {
    let fib = Fib::new();
    let result: u64 = fib.take_while(|x| *x < 4000000).filter(|x| *x % 2 == 0).sum();
    println!("{}", result);
}
