fn check_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}
// fn main() {
//     let n = 13;
//     println!("{}", check_prime(n));
// }

