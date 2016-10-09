extern crate primes;
use primes::Primes;

fn main() {
    let primes = Primes::new();
    let num = 600851475143;
    let num_f = 600851475143.0_f64;
    let sqrt_num = num_f.sqrt();


    let result: u64 = primes.take_while(|x| *x <= (sqrt_num.ceil() as u64))
        .filter(|x| num % *x == 0)
        .max()
        .unwrap();
    println!("{}", result);
}
