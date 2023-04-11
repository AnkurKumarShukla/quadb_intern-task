// Given a sorted array of integers, implement a function that returns the median of the array.
fn median_of_sorted_array(a: &[i32]) -> f64 {
    let len = a.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (a[mid] + a[mid - 1]) as f64 / 2.0
    } else {
        a[len / 2] as f64
    }
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{}", median_of_sorted_array(&a));
}