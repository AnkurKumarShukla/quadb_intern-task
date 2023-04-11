// Merge two sorted arrays in Rust
fn merge_sorted_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut merged = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }
    while i < a.len() {
        merged.push(a[i]);
        i += 1;
    }
    while j < b.len() {
        merged.push(b[j]);
        j += 1;
    }
    merged
}

fn main() {
    let a = [1, 3, 5, 7, 9];
    let b = [2, 4, 6, 8, 10];
    println!("{:?}", merge_sorted_arrays(&a, &b));
}