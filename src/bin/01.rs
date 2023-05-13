pub fn part_one(input: &str) -> Option<u32> {
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut sums: Vec<u32> = vec![];

    for elf in &elves {
        let mut sum: u32 = 0;
        for line in elf.lines() {
            sum += line.parse::<u32>().unwrap();
        }
        sums.push(sum);
    }

    let answer = sums.iter().max().unwrap();

    Some(*answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sums: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect();

    sums.sort_unstable();
    sums.reverse();

    let total_last_3 = sums[..=2].iter().sum();

    Some(total_last_3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
