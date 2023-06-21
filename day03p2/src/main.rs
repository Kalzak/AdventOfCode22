use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read to string");

    let mut all_common_items: Vec<char> = Vec::new();

    let mut three_group: Vec<&str> = Vec::new();

    let mut badges: Vec<char> = Vec::new();

    for line in contents.lines() {
        let (comp1, comp2) = line.split_at(line.len()/2);
        find_common_items(comp1, comp2, &mut all_common_items);

        three_group.push(line);

        if three_group.len() == 3 {
            let mut map: HashMap<char, i32> = HashMap::new();

            for elfcontents in &three_group {
                let mut single_elfchars: Vec<char> = elfcontents.chars().collect();
                single_elfchars.sort();
                single_elfchars.dedup();
                for character in single_elfchars {
                    let old = match map.get(&character) {
                        Some(count) => count + 1,
                        None => 1
                    };
                    map.insert(
                        character,
                        old
                    );
                }
            }

            for key in map.keys() {
                let potential_badge = map.get(key).unwrap();
                if potential_badge == &3 {
                    badges.push(*key);
                    break;
                }
            }
            three_group.clear();
        }

    }

    let mut priority_sum = 0;

    for item in all_common_items {
        priority_sum += get_priority(item);
    }

    let mut badge_priority_sum = 0;

    for item in badges {
        badge_priority_sum += get_priority(item);
    }
    
    println!("Priority sum: {}", priority_sum);
    println!("Priority sum: {}", badge_priority_sum);

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