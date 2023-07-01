use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read to string");

    let mut total_subset_ranges: i32 = 0;
    let mut total_overlap_ranges: i32 = 0;

    for line in contents.lines() {
        let mut pair_assignments: Vec<&str> = line.split(",").collect();

        let mut split = pair_assignments[0].split("-");
        let elf1_start: i32 = split.next().unwrap().parse().unwrap();
        let elf1_end: i32 = split.next().unwrap().parse().unwrap();

        let mut split = pair_assignments[1].split("-");
        let elf2_start: i32 = split.next().unwrap().parse().unwrap();
        let elf2_end: i32 = split.next().unwrap().parse().unwrap();

        let elf1 = (elf1_start, elf1_end);
        let elf2 = (elf2_start, elf2_end);

        println!("{:?} {:?}", elf1, elf2);

        if check_subset(elf1, elf2) || check_subset(elf2, elf1) {
            println!("subset found");
            total_subset_ranges += 1;
        }

        if check_overlap(elf1, elf2) || check_overlap(elf2, elf1) {
            println!("overlap found");
            total_overlap_ranges += 1;
        }

    }

    println!("Total subsets: {}", total_subset_ranges);
    println!("Total overlaps: {}", total_overlap_ranges);
}

fn check_subset(elf1: (i32, i32), elf2: (i32, i32)) -> bool {
    if elf1.0 <= elf2.0 && elf1.1 >= elf2.1 {
        return true;
    }
    false
}

fn check_overlap(elf1: (i32, i32), elf2: (i32, i32)) -> bool {
    if elf1.0 <= elf2.0 && elf1.1 >= elf2.0 {
        return true;
    }
    if elf1.0 <= elf2.1 && elf1.1 >= elf2.1 {
        return true;
    }
    false
}