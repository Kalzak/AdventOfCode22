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

#[derive(Debug)]
#[derive(PartialEq)]
enum Outcome {
    Loss,
    Draw,
    Win,
    Invalid
}

/*

Wrong outputs:
12853

*/

fn main() {
    let mut file = File::open("input.txt").expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read to string");

    let mut total = 0;

    for line in contents.lines() {
        let actionoutcome: Vec<&str> = line.split_whitespace().collect();

        let opmove = process_move(actionoutcome[0]);
        let outcome = process_outcome(actionoutcome[1]);
        let mymove = calc_move(&opmove, &outcome);


        let points = calc_points(&mymove, &opmove);

        println!("{:?} {:?} {:?} {:?}", &opmove, &outcome, &mymove, &points);

        total += points;

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

fn process_outcome(string_outcome: &str) -> Outcome {
    match string_outcome {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => Outcome::Invalid
    }
}

fn calc_move(opmove: &Move, outcome: &Outcome) -> Move {
    match opmove {
        Move::Paper => {
            match outcome {
                Outcome::Loss => Move::Rock,
                Outcome::Draw => Move::Paper,
                Outcome::Win => Move::Scissors,
                _ => Move::Invalid
            }
        },
        Move::Scissors => {
            match outcome {
                Outcome::Loss => Move::Paper,
                Outcome::Draw => Move::Scissors,
                Outcome::Win => Move::Rock,
                _ => Move::Invalid
            }
        },
        Move::Rock => {
            match outcome {
                Outcome::Loss => Move::Scissors,
                Outcome::Draw => Move::Rock,
                Outcome::Win => Move::Paper,
                _ => Move::Invalid
            }
        },
        _ => Move::Invalid
    }
}


fn calc_points(mymove: &Move, opmove: &Move) -> i32 {
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
