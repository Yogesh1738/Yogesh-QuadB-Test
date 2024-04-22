fn lcp(strs: Vec<String>) -> String {
    let mut ans = String::new();

    for (i, c) in strs[0].chars().enumerate() {
        for j in 1..strs.len() {
            if let Some(ch) = strs[j].chars().nth(i) {
                if c != ch {
                    return ans;
                }
            } else {
                return ans;
            }
        }
        ans.push(c);
    }

    return ans;
}

// fn main() {
//     let arr = vec!["abcderefe".to_string(), "abcdeecdde".to_string(), "abcrrgrg".to_string(), "abcrft".to_string()];
//     println!("{}", lcp(arr));
// }

