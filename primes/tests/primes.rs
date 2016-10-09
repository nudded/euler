extern crate primes;

use primes::Primes;
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
