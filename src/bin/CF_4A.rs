use std::io;
fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Input should be valid UTF-8 string");
    let w = buf
        .trim()
        .parse::<u32>()
        .expect("Input should be an unsigned integer");
    if w % 2 == 0 && w > 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
