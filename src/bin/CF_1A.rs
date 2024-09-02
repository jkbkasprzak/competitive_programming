use std::io;
fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Input should be valid UTF-8 string");

    let numbers: Vec<u32> = buf
        .trim()
        .split(' ')
        .map(|x| {
            x.parse::<u32>()
                .expect("Input should be an unsigned integer")
        })
        .collect();
    if numbers.len() != 3 {
        panic!("Expects 3 unsigned integers on input")
    }
    let result = (numbers[0].div_ceil(numbers[2]) as u64)
        .checked_mul(numbers[1].div_ceil(numbers[2]) as u64)
        .expect("Result should fit into u64");
    println!("{result}");
}
