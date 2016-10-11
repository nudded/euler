use std::collections::HashMap;

pub struct Primes {
    primes: HashMap<u64, u64>,
    curr: u64,
}

pub trait PrimeInfo {
    fn prime_factors(&mut self) -> Vec<u64>;
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

impl PrimeInfo for u64 {
    fn prime_factors(&mut self) -> Vec<u64> {
        let mut primes = Primes::new();
        let mut result = Vec::new();
        let mut num = *self;
        let mut cur_prime = primes.next().unwrap();
        while num > 1_u64 {
            while (num % cur_prime) != 0 {
                cur_prime = primes.next().unwrap();
            }
            num = num / cur_prime;
            result.push(cur_prime);
        }
        result
    }
}
