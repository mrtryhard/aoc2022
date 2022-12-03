use std::fs;

const WIN_PTS: u32 = 6;
const DRAW_PTS: u32 = 3;
const LOSE_PTS: u32 = 0;

const ROCK_PTS: u32 = 1;
const PAPER_PTS: u32 = 2;
const SCISSORS_PTS: u32 = 3;

trait Scorer {
    fn get_moves_score(&self, opponent_move: &str, my_move: &str) -> u32;
}

struct MoveScorer {}
struct OutcomeScorer {}

/*
   Gets the game score according to the following rules:
       o A=Rock, B=Paper, C=Scissors
       o X=Rock, Y=Paper, Z=Scissors
*/
impl Scorer for MoveScorer {
    fn get_moves_score(&self, opponent_move: &str, my_move: &str) -> u32 {
        match (opponent_move, my_move) {
            ("A", "X") => ROCK_PTS + DRAW_PTS,
            ("A", "Y") => PAPER_PTS + WIN_PTS,
            ("A", "Z") => SCISSORS_PTS + LOSE_PTS,

            ("B", "X") => ROCK_PTS + LOSE_PTS,
            ("B", "Y") => PAPER_PTS + DRAW_PTS,
            ("B", "Z") => SCISSORS_PTS + WIN_PTS,

            ("C", "X") => ROCK_PTS + WIN_PTS,
            ("C", "Y") => PAPER_PTS + LOSE_PTS,
            ("C", "Z") => SCISSORS_PTS + DRAW_PTS,
            _ => 0,
        }
    }
}

/*
   Gets the game score according to the following rules:
       o A=Rock, B=Paper, C=Scissors
       o X=Lose, Y=Draw, Z=Win
*/
impl Scorer for OutcomeScorer {
    fn get_moves_score(&self, opponent_move: &str, my_move: &str) -> u32 {
        match (opponent_move, my_move) {
            ("A", "X") => SCISSORS_PTS + LOSE_PTS,
            ("A", "Y") => ROCK_PTS + DRAW_PTS,
            ("A", "Z") => PAPER_PTS + WIN_PTS,

            ("B", "X") => ROCK_PTS + LOSE_PTS,
            ("B", "Y") => PAPER_PTS + DRAW_PTS,
            ("B", "Z") => SCISSORS_PTS + WIN_PTS,

            ("C", "X") => PAPER_PTS + LOSE_PTS,
            ("C", "Y") => SCISSORS_PTS + DRAW_PTS,
            ("C", "Z") => ROCK_PTS + WIN_PTS,
            _ => 0,
        }
    }
}

fn get_tournament_score(input: &str, scorer: &dyn Scorer) -> u32 {
    let games = input.split("\r\n");

    games
        .map(|game| {
            let moves = game.split(" ").collect::<Vec<&str>>();
            scorer.get_moves_score(moves.get(0).unwrap(), moves.get(1).unwrap())
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Needs an input file.");

    println!("As move: {}", get_tournament_score(&input, &MoveScorer {}));
    println!(
        "As outcome: {}",
        get_tournament_score(&input, &OutcomeScorer {})
    );
}

#[cfg(test)]
mod tests {
    use crate::{get_tournament_score, MoveScorer, OutcomeScorer};
    use std::fs;

    #[test]
    fn tst_find_tournament_score_for_letter_as_move() {
        let input = fs::read_to_string("tst_input.txt").expect("Needs an input file.");

        assert_eq!(get_tournament_score(&input, &MoveScorer {}), 15);
    }

    #[test]
    fn tst_find_tournament_score_for_letter_as_outcome() {
        let input = fs::read_to_string("tst_input.txt").expect("Needs an input file.");

        assert_eq!(get_tournament_score(&input, &OutcomeScorer {}), 12);
    }
}
