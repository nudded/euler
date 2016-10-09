use std::collections::HashMap;

pub struct Primes {
    primes: HashMap<u64, u64>,
    curr: u64,
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            curr: 1,
            primes: HashMap::new(),
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let mut found = false;
        while !found {
            self.curr += 1;
            match self.primes.get(&self.curr) {
                None => {
                    self.primes.insert(self.curr * self.curr, self.curr);
                    found = true
                }
                Some(&p) => {
                    let mut x = p + self.curr;
                    while self.primes.contains_key(&x) {
                        x += p
                    }
                    self.primes.insert(x, p);
                }
            }
        }
        Some(self.curr)
    }
}
