enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn get_score(&self) -> u32 {
        match self {
            Outcome::Draw => 3,
            Outcome::Win => 6,
            Outcome::Lose => 0,
        }
    }

    fn determine(player1: Rps, player2: Rps) -> Self {
        match (player1, player2) {
            (Rps::Rock, Rps::Paper) | (Rps::Paper, Rps::Scissors) | (Rps::Scissors, Rps::Rock) => {
                Outcome::Win
            }
            (Rps::Rock, Rps::Scissors) | (Rps::Paper, Rps::Rock) | (Rps::Scissors, Rps::Paper) => {
                Outcome::Lose
            }
            (Rps::Rock, Rps::Rock) | (Rps::Paper, Rps::Paper) | (Rps::Scissors, Rps::Scissors) => {
                Outcome::Draw
            }
        }
    }
}

enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn get_score(&self) -> u32 {
        match &self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
}

fn calculate_score(opponent: Rps, desired_outcome: Outcome) -> u32 {
    let score = desired_outcome.get_score()
        + match (&opponent, &desired_outcome) {
            (Rps::Rock, Outcome::Win) => Rps::Paper.get_score(),
            (Rps::Rock, Outcome::Lose) => Rps::Scissors.get_score(),
            (Rps::Rock, Outcome::Draw) => Rps::Rock.get_score(),

            (Rps::Paper, Outcome::Win) => Rps::Scissors.get_score(),
            (Rps::Paper, Outcome::Lose) => Rps::Rock.get_score(),
            (Rps::Paper, Outcome::Draw) => Rps::Paper.get_score(),

            (Rps::Scissors, Outcome::Win) => Rps::Rock.get_score(),
            (Rps::Scissors, Outcome::Lose) => Rps::Paper.get_score(),
            (Rps::Scissors, Outcome::Draw) => Rps::Scissors.get_score(),
        };

    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let rounds: Vec<&str> = input.trim().split("\n").collect();

    let mut score: u32 = 0;
    for round in &rounds {
        let round: Vec<char> = round
            .split(" ")
            .map(|ch| ch.parse::<char>().unwrap())
            .collect();

        let opponent = match &round[0] {
            'A' => Some(Rps::Rock),
            'B' => Some(Rps::Paper),
            'C' => Some(Rps::Scissors),
            _ => None,
        }
        .unwrap();

        let me = match &round[1] {
            'X' => Some(Rps::Rock),
            'Y' => Some(Rps::Paper),
            'Z' => Some(Rps::Scissors),
            _ => None,
        }
        .unwrap();

        score += me.get_score() + Outcome::determine(opponent, me).get_score();
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rounds: Vec<&str> = input.trim().split("\n").collect();

    let mut score: u32 = 0;

    for round in &rounds {
        let round: Vec<char> = round
            .split(" ")
            .map(|ch| ch.parse::<char>().unwrap())
            .collect();

        let opponent = match &round[0] {
            'A' => Some(Rps::Rock),
            'B' => Some(Rps::Paper),
            'C' => Some(Rps::Scissors),
            _ => None,
        }
        .unwrap();

        let desired_outcome = match &round[1] {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None,
        }
        .unwrap();

        score += calculate_score(opponent, desired_outcome)
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
