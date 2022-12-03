enum MatchRes {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl MatchRes {
    fn from_outcome(me: Option<&str>) -> MatchRes {
        use MatchRes::*;
        match me {
            Some("X") => Loss,
            Some("Y") => Draw,
            Some("Z") => Win,
            _ => panic!("Not x, y, or z"),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    fn match_result(me: &Choice, them: &Choice) -> MatchRes {
        use Choice::*;
        use MatchRes::*;
        match (me, them) {
            (m, t) if m == t => Draw,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Scissors, Paper) => Win,
            (Scissors, Rock) => Loss,
            (Rock, Paper) => Loss,
            (Paper, Scissors) => Loss,
            _ => panic!("Not a matched pattern somehow"),
        }
    }

    fn for_me(res: Option<&str>) -> Self {
        use Choice::*;
        match res {
            Some("X") => Rock,
            Some("Y") => Paper,
            Some("Z") => Scissors,
            _ => panic!("Not X, Y, or Z, was {}", res.unwrap_or_default()),
        }
    }

    fn for_them(res: Option<&str>) -> Self {
        use Choice::*;
        match res {
            Some("A") => Rock,
            Some("B") => Paper,
            Some("C") => Scissors,
            _ => panic!("Not A, B, or C"),
        }
    }

    fn from_outcome(my_turn: &MatchRes, them: &Choice) -> Self {
        use Choice::*;
        use MatchRes::*;
        match (my_turn, them) {
            (Loss, Rock) => Scissors,  // lose
            (Loss, Scissors) => Paper, // lose
            (Loss, Paper) => Rock,     // lose
            (Win, Rock) => Paper,      // lose
            (Win, Scissors) => Rock,   // lose
            (Win, Paper) => Scissors,  // lose
            (Draw, &t) => t,           // lose
        }
    }
}

fn round_score_part_one(round: &str) -> u8 {
    let mut turns: Vec<_> = round.split(' ').collect();
    let me = Choice::for_me(turns.pop());
    let them = Choice::for_them(turns.pop());
    Choice::match_result(&me, &them) as u8 + me as u8
}

fn round_score_part_two(round: &str) -> u8 {
    let mut turns: Vec<_> = round.split(' ').collect();
    let turn_res = MatchRes::from_outcome(turns.pop());
    let them = Choice::for_them(turns.pop());
    let me = Choice::from_outcome(&turn_res, &them);
    turn_res as u8 + me as u8
}

fn main() {
    let results = std::fs::read_to_string("src/day02.txt").unwrap();

    println!(
        "Part two: {}",
        results
            .lines()
            .fold(0, |score, round| score + round_score_part_one(round) as u32)
    );
    println!(
        "Part two: {}",
        results
            .lines()
            .fold(0, |score, round| score + round_score_part_two(round) as u32)
    );
}
