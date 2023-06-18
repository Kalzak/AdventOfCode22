use std::fs::File;
use std::io::Read;

#[derive(Debug)]
#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
    Invalid
}

fn main() {
    let mut file = File::open("input.txt").expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read to string");

    let mut total = 0;

    for line in contents.lines() {
        let actions: Vec<&str> = line.split_whitespace().collect();

        let mymove = process_move(actions[1]);
        let opmove = process_move(actions[0]);
        total += calc_points(mymove, opmove);
    }
    
    println!("{}", total);
}

fn process_move(string_move: &str) -> Move {
    match string_move {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => Move::Invalid
    }
}

fn calc_points(mymove: Move, opmove: Move) -> i32 {
    let move_points = match mymove {
        Move::Rock => 1, 
        Move::Paper => 2,
        Move::Scissors => 3,
        _ => 0
    };
    if mymove == opmove {
        return move_points + 3;
    }
    let outcome = match opmove {
        Move::Rock => {
            match mymove {
                Move::Paper => 6,
                _ => 0
            }
        },
        Move::Paper => {
            match mymove {
                Move::Scissors => 6,
                _ => 0
            }
        },
        Move::Scissors => {
            match mymove {
                Move::Rock => 6,
                _ => 0
            }
        },
        _ => 0
    };
    return move_points + outcome;
}
