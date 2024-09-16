use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Input should be valid UTF-8 string");
    let test_cases: u32 = buf
        .trim()
        .parse()
        .expect("First line shoud be unsigned integer");

    for _i in 0..test_cases {
        buf.clear();
        io::stdin()
            .read_line(&mut buf)
            .expect("Input should be valid UTF-8 string");
        let row_count: u32 = buf
            .trim()
            .parse()
            .expect("Test case shoud begin with unsigned integer");
        let mut positions = Vec::new();
        for _j in 0..row_count {
            buf.clear();
            io::stdin()
                .read_line(&mut buf)
                .expect("Input should be valid UTF-8 string");
            positions.push(1 + buf.find('#').expect("Row should contain note (#)"));
        }
        for pos in positions.iter().rev() {
            print!("{pos} ")
        }
        println!();
    }
}
