// Palindrome check in Rust:
fn is_palindrome(s: &str) -> bool {
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s.chars().nth(i) != s.chars().nth(j) {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

// method 2 
fn is_palindrome2(s: &str) -> bool {
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}



fn main() {
    let s = "racecar";
    println!("{} is a palindrome: {}", s, is_palindrome(s));
    println!("{} is a palindrome: {}", s, is_palindrome2(s));
}