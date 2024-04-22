fn fun(s: &mut String) {
    let mut i = 0;
    let mut j = s.len() - 1;
    let bytes = unsafe { s.as_bytes_mut() };

    while i <= j {

        let temp = bytes[i];
        bytes[i] = bytes[j];
        bytes[j] = temp;
        i += 1;
        j -= 1;
    }
}
// fn main() {
//     let mut s = String::from("hello");
//     fun(&mut s);
//     println!("{}", s);
// }
