// Implement a function that returns the kth smallest element in a given array.

fn kth_smallest(a: &[i32], k: usize) -> i32 {
    let mut a = a.to_vec();
    a.sort();
    a[k - 1]
}

// method2
fn kth_smallest2(arr: &[i32], k: usize) -> Option<i32> {
    arr.iter().copied().nth(k - 1)
}


fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{}", kth_smallest(&a, 3));
    println!("{:?}", kth_smallest2(&a, 3));
}