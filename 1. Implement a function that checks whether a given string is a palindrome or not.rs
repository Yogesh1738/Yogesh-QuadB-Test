fn check_palindrome(s: &str) -> bool {
    let mut chars = s.chars();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i <= j {
        if chars.next() != s.chars().rev().nth(i) {
            return false;
        }
        i += 1;
        j -= 1;
    }
    return true;
}

// fn main() {
//     let s = String::from("levelefewfe");
//     println!("{} {}", s, check_palindrome(&s));
// }
