#[derive(Clone, Copy)]
enum Call {
    Rock,
    Paper,
    Scissors,
}

enum Response {
    Rock,
    Paper,
    Scissors,
}

enum DesiredOutcome {
    Lose,
    Draw,
    Win,
}

impl From<&str> for Call {
    fn from(c: &str) -> Call {
        match c {
            "A" => Call::Rock,
            "B" => Call::Paper,
            "C" => Call::Scissors,
            _ => panic!("lol char"),
        }
    }
}

impl From<&str> for Response {
    fn from(c: &str) -> Response {
        match c {
            "X" => Response::Rock,
            "Y" => Response::Paper,
            "Z" => Response::Scissors,
            _ => panic!("lol char"),
        }
    }
}
impl From<&str> for DesiredOutcome {
    fn from(c: &str) -> DesiredOutcome {
        match c {
            "X" => DesiredOutcome::Lose,
            "Y" => DesiredOutcome::Draw,
            "Z" => DesiredOutcome::Win,
            _ => panic!("lol char"),
        }
    }
}

fn score_part1(c: Call, r: Response) -> i32 {
    match (c, r) {
        (Call::Rock, Response::Rock) => 1 + 3,
        (Call::Rock, Response::Paper) => 2 + 6,
        (Call::Rock, Response::Scissors) => 3 + 0,
        (Call::Paper, Response::Rock) => 1 + 0,
        (Call::Paper, Response::Paper) => 2 + 3,
        (Call::Paper, Response::Scissors) => 3 + 6,
        (Call::Scissors, Response::Rock) => 1 + 6,
        (Call::Scissors, Response::Paper) => 2 + 0,
        (Call::Scissors, Response::Scissors) => 3 + 3,
    }
}

fn score_part2(c: Call, d: DesiredOutcome) -> Response {
    match (c, d) {
        (Call::Rock, DesiredOutcome::Lose) => Response::Scissors,
        (Call::Rock, DesiredOutcome::Draw) => Response::Rock,
        (Call::Rock, DesiredOutcome::Win) => Response::Paper,
        (Call::Paper, DesiredOutcome::Lose) => Response::Rock,
        (Call::Paper, DesiredOutcome::Draw) => Response::Paper,
        (Call::Paper, DesiredOutcome::Win) => Response::Scissors,
        (Call::Scissors, DesiredOutcome::Lose) => Response::Paper,
        (Call::Scissors, DesiredOutcome::Draw) => Response::Scissors,
        (Call::Scissors, DesiredOutcome::Win) => Response::Rock,
    }
}
fn main() {
    let contents = std::fs::read_to_string("inputs/2.txt").expect("read input");

    let score: i32 = contents
        .split_terminator("\n")
        .map(|game| {
            let mut i = game.split_ascii_whitespace();
            let c: Call = i.next().unwrap().into();
            let r: Response = i.next().unwrap().into();
            score_part1(c, r)
        })
        .sum();

    let score: i32 = contents
        .split_terminator("\n")
        .map(|game| {
            let mut i = game.split_ascii_whitespace();
            let c: Call = i.next().unwrap().into();
            let d: DesiredOutcome = i.next().unwrap().into();
            score_part1(c, score_part2(c, d))
        })
        .sum();

    println!("score: {}", score)
}
