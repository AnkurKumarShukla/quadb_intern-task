// Find the maximum subarray sum in Rust:
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = 0;
    for i in 0..arr.len() {
        current_sum += arr[i];
        if current_sum < 0 {
            current_sum = 0;
        }
        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }
    max_sum
}


fn main(){
    let arr = [1, 2, 3, -2, 5];
    println!("{}", max_subarray_sum(&arr));
    println!("{}", max_subarray_sum2(&arr));
}