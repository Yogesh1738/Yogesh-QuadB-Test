fn first_occurrence(arr: &[i32], value: i32) -> i32 {
    let mut ans = -1;
    let n = arr.len();

    if n > 0 && arr[0] == value {
        return 0;
    }

    let mut start = 0;
    let mut end = n - 1;

    while start <= end {
        let mid = start + (end - start) / 2;
        if arr[mid] == value {
            let mut first = mid as i32;
            while first >= 0 && arr[first as usize] == value {
                ans = first;
                first -= 1;
            }
            return ans;
        } else if arr[mid] < value {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    return ans;
}
// fn main() {
//     let arr = [2, 2, 4, 4, 4, 4, 4, 6, 7, 8, 9];
//     println!("{}", first_occurrence(&arr, 4));
// }

