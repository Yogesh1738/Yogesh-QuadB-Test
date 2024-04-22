use std::cmp;

fn maxSubarraySum(arr: &[i32]) -> i32 {
    let mut ans = std::i32::MIN;
    let mut sum = 0;

    for &num in arr {
        sum += num;
        if num >= sum {
            sum = num;
        }
        ans = cmp::max(ans, sum);
    }

    return ans;
}
// fn main() {
//     let arr = [1, -2, 3, -4, 5, -6, 7];
//     println!("{}", maxSubarraySum(&arr));
// }