fn longest_common_prefix<'a>(strings: &'a [&'a str]) -> &'a str {
    if strings.is_empty() {
        return "";
    }
    let mut prefix = strings[0];
    for s in &strings[1..] {
        while !s.starts_with(prefix) {
            prefix = &prefix[..prefix.len() - 1];
            if prefix.is_empty() {
                return "";
            }
        }
    }
    prefix
}




fn main() {
    let strings = ["apple", "ape", "april"];
    let prefix = longest_common_prefix(&strings);
    println!("{}", prefix); // Output: "ap"
}
