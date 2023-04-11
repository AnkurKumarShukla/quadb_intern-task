// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn find_first_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i < j {
        let mid = (i + j) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            i = mid + 1;
        } else {
            j = mid - 1;
        }
    }
    None
}


// method 2 
fn find_first_index2(arr: &[i32], target: i32) -> Option<usize> {
    arr.binary_search(&target).ok()
}


fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", find_first_index(&arr, 5));
    println!("{:?}", find_first_index2(&arr, 5));
}