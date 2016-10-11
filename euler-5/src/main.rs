extern crate primes;

use primes::PrimeInfo;

fn main() {
    let mut common_divisors: Vec<u64> = Vec::new();
    for x in 2..21 {
        let mut i = 0;
        for p in (x as u64).prime_factors() {
            match common_divisors.iter().skip(i).position(|v| *v == p) {
                Some(new_i) => i += new_i + 1,
                None => {
                    common_divisors.push(p);
                    i = 0
                }
            }
        }
    }

    let result: u64 = common_divisors.iter().product();
    println!("{}", result);
}
