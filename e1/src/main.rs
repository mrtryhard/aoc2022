use std::fs;

fn get_most_calories(input: &str) -> u32 {
    let elves_items = input.split("\r\n\r\n");

    let a = elves_items.map(|elve_items| elve_items.split("\r\n")
            .map(|item| item.parse::<u32>().unwrap_or(0))
            .sum())
        .max()
        .unwrap_or(0);

    a
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Needs an input file.");

    println!("{}", get_most_calories(input.as_str()))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::get_most_calories;

    #[test]
    fn tst_finds_elf_with_most_calories() {
        let input = fs::read_to_string("tst_input.txt").expect("Needs an input file.");

        assert_eq!(get_most_calories(&input), 24000);
    }
}