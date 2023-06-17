use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read to string");

    let mut best: i32 = 0;
    let mut current: i32 = 0;

    for line in contents.lines() {
        if line != "" {
            let value: i32 = line.parse().unwrap();
            current += value;
        } else {
            if current > best {
                best = current;
            }
            current = 0;
        }
    }

    println!("{}", best);
}