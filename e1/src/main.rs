use std::fs;

fn get_calories_per_elves(input: &str) -> Vec<u32> {
    let elves_items = input.split("\r\n\r\n");

    elves_items
        .map(|elf_items| {
            elf_items
                .split("\r\n")
                .map(|item| item.parse::<u32>().unwrap_or(0))
                .sum()
        })
        .collect::<Vec<u32>>()
}

fn get_top_three_most_calories(input: &str) -> u32 {
    let mut calories_per_elf = get_calories_per_elves(input);
    calories_per_elf.sort_by(|a, b| b.cmp(a));
    calories_per_elf.truncate(3);
    calories_per_elf.iter().sum()
}

fn get_most_calories(input: &str) -> u32 {
    get_calories_per_elves(input).iter().max().unwrap().clone()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Needs an input file.");

    println!("Most calories: {}", get_most_calories(input.as_str()));
    println!(
        "Sum of top 3 most calories: {}",
        get_top_three_most_calories(input.as_str())
    );
}

#[cfg(test)]
mod tests {
    use crate::{get_most_calories, get_top_three_most_calories};
    use std::fs;

    #[test]
    fn tst_finds_elf_with_most_calories() {
        let input = fs::read_to_string("tst_input.txt").expect("Needs an input file.");

        assert_eq!(get_most_calories(&input), 24000);
    }

    #[test]
    fn tst_sum_of_top_3_elves_with_most_calories() {
        let input = fs::read_to_string("tst_input.txt").expect("Needs an input file.");

        assert_eq!(get_top_three_most_calories(&input), 45000);
    }
}
