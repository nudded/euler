extern crate primes;

use primes::PrimeInfo;
use std::collections::HashMap;

fn main() {
    let mut common_divisors: HashMap<u64, u64> = HashMap::new();

    for x in 2..21 {
        let prime_fact = (x as u64).prime_factors();
        let mut count_map: HashMap<u64, u64> = HashMap::new();
        for x in prime_fact {
            *count_map.entry(x).or_insert(0) += 1
        }

        for (k, v) in count_map {
            let entry = common_divisors.entry(k).or_insert(v);
            if *entry < v {
                *entry = v
            }
        }
    }

    let result: u64 = common_divisors.iter().map(|(k, v)| k.pow(*v as u32)).product();
    println!("{}", result);
}
