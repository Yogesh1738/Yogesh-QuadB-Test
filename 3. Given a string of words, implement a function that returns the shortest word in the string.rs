fn fun(s: &str) -> String {
    let mut ans = String::new();
    let mut l = std::usize::MAX;
    let mut i = 0;
    for (j, c) in s.chars().enumerate() {
        if c == ' ' {
            let temp = &s[i..j];
            if l > temp.len() {
                l = temp.len();
                ans = temp.to_string();
            }
            i = j + 1;
        }
    }
    return ans;
}

// fn main() {
//     let s = String::from("hello yogi iffwfs affref werewd");
//     println!("{}", fun(&s));
// }
