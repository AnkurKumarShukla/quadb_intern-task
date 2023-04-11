// Reverse a string in Rust

fn revstr(s: &str) -> String {
    let mut rev = String::new();
    for c in s.chars().rev() {
        rev.push(c);
    }
    rev
}

// method2
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}



fn main() {
    let s = "Hello, world!";
    println!("{}", revstr(s));
    println!("{}", reverse_string(s));
}