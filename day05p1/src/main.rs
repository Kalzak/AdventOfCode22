use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    // Figure out how many stacks
    let num_stacks = (lines[0].len() + 1) / 4;

    // Read initial stack data
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    let mut curr_stack_depth = 0;

    'outer: loop {
        for i in 0..num_stacks {
            let item = lines[curr_stack_depth].chars().nth(1 + 4 * i).unwrap();
            if item.is_numeric() {
                break 'outer;
            }
            if item.is_alphabetic() {
                stacks[i].insert(0, item);
            }
        }
        curr_stack_depth += 1;
    }
    //println!("{:#?}", stacks);

    // Process move data
    let move_start_idx = curr_stack_depth + 2;
    for i in move_start_idx..lines.len() {
        let words: Vec<&str> = lines[i].split_whitespace().collect();
        let num_to_move: usize = words[1].parse().unwrap();
        let src_stack: usize = words[3].parse().unwrap();
        let dest_stack: usize = words[5].parse().unwrap();

        for _ in 0..num_to_move {
            let item: char = stacks[src_stack - 1].pop().unwrap();
            stacks[dest_stack - 1].push(item);
        }

        //break;
    }

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }
    print!("\n");
}
