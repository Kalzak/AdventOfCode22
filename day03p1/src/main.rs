use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read to string");

    let mut all_common_items: Vec<char> = Vec::new();

    for line in contents.lines() {
        let (comp1, comp2) = line.split_at(line.len()/2);
        println!("{} {}", comp1, comp2);
        find_common_items(comp1, comp2, &mut all_common_items);
    }

    println!("{:?}", all_common_items);
    
    let mut priority_sum = 0;

    for item in all_common_items {
        priority_sum += get_priority(item);
    }
    
    println!("{}", priority_sum);

}

fn find_common_items(comp1: &str, comp2: &str, all_common_items: &mut Vec<char>) {
    let comp1_chars: Vec<char> = comp1.chars().collect();
    let mut common_items: Vec<char> = Vec::new();
    for comp1_char in comp1_chars {
        if comp2.contains(comp1_char) {
            common_items.push(comp1_char);
        }
    }
    common_items.sort();
    common_items.dedup();
    all_common_items.append(&mut common_items);
}

fn get_priority(item: char) -> i32 {
    let is_upper_case = item.is_uppercase();
    let mut item = item.to_ascii_uppercase() as u8;
    item -= 64;
    if is_upper_case {
        item += 26;
    }
    return item as i32;
}