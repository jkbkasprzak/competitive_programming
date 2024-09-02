use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Input should be valid UTF-8 string");
    let message = buf.trim();
    let size = message.chars().count();
    let mut merge_detected = false;
    for i in size / 2 + 1..size {
        let first_word = &message[..i];
        let second_word = &message[size - i..size];
        if first_word == second_word {
            println!("Yes");
            println!("{first_word}");
            merge_detected = true;
            break;
        }
    }
    if !merge_detected {
        println!("No")
    }
}
