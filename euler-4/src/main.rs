use std::fmt::Display;

trait IsPalindrome {
    fn is_palindrome(&self) -> bool;
}

trait IsPalindromeString {
    fn is_palindrome_internal(&self) -> bool;
}

impl<T: Display> IsPalindrome for T {
    fn is_palindrome(&self) -> bool {
        self.to_string().is_palindrome_internal()
    }
}

impl IsPalindromeString for String {
    fn is_palindrome_internal(&self) -> bool {
        self.chars().zip(self.chars().rev()).all(|(x, y)| x == y)
    }
}

fn main() {
    let mut max = 0u64;
    for x in 100..1000 {
        for y in 100..1000 {
            if (x * y).is_palindrome() && x * y > max {
                max = x * y;
            }
        }
    }
    println!("{}", max);
}
