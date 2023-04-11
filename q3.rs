// Given a string of words, implement a function that returns the shortest word in the string.

fn shortest_word(s: &str) -> &str {
    let mut shortest = s;
    for word in s.split_whitespace() {
        if word.len() < shortest.len() {
            shortest = word;
        }
    }
    shortest
}

// method 2
fn shortest_word2(s: &str) -> Option<&str> {
   s.split_whitespace().min_by_key(|word| word.len())

   
}


fn main() {
    let s = "I am a string of words";
    println!("{}", shortest_word(s));
    println!("{:?}", shortest_word2(s));
}