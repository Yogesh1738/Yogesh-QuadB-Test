use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn kth_smallest(arr: &[i32], k: usize) -> i32 {
    let mut q = BinaryHeap::new();
    for &num in arr {
        q.push(Reverse(num));
    }
    for _ in 0..k - 1 {
        q.pop();
    }
    q.pop().unwrap().0
}
// fn main() {
//     let arr = [1, 5, 4, 3, 2, 7, 8, 9, 0];
//     println!("{}", kth_smallest(&arr, 6));
// }