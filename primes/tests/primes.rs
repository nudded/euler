extern crate primes;

use primes::Primes;
use primes::PrimeInfo;
#[test]
fn first_primes() {
    let primes = Primes::new();
    let mut first_primes = primes.take(10);
    assert_eq!(first_primes.next().unwrap(), 2);
    assert_eq!(first_primes.next().unwrap(), 3);
    assert_eq!(first_primes.next().unwrap(), 5);
    assert_eq!(first_primes.next().unwrap(), 7);
    assert_eq!(first_primes.next().unwrap(), 11);
    assert_eq!(first_primes.next().unwrap(), 13);
    assert_eq!(first_primes.next().unwrap(), 17);
    assert_eq!(first_primes.next().unwrap(), 19);
    assert_eq!(first_primes.next().unwrap(), 23);
}

#[test]
fn prime_factors() {
    assert_eq!(17u64.prime_factors(), [17]);
    assert_eq!(25u64.prime_factors(), [5, 5]);
    assert_eq!(2u64.prime_factors(), [2]);
    assert_eq!(8u64.prime_factors(), [2, 2, 2]);
    assert_eq!(24768u64.prime_factors(), [2, 2, 2, 2, 2, 2, 3, 3, 43]);
}
