// Implement a function that checks whether a given number is prime or not.
fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", is_prime(1));
    println!("{}", is_prime(2));
    println!("{}", is_prime(3));
    println!("{}", is_prime(4));

}