fn fun(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < v1.len() && j < v2.len() {
        if v1[i] <= v2[j] {
            ans.push(v1[i]);
            i += 1;
        } else {
            ans.push(v2[j]);
            j += 1;
        }
    }

    while i < v1.len() {
        ans.push(v1[i]);
        i += 1;
    }

    while j < v2.len() {
        ans.push(v2[j]);
        j += 1;
    }

    return ans;
}


// fn main() {
//     let v1 = vec![1, 2, 3, 4, 5, 6, 7];
//     let v2 = vec![12, 13, 34, 45, 56];
//     let ans = fun(v1, v2);
//
//     for val in ans {
//         print!("{} ", val);
//     }
// }