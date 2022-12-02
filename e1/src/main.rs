fn get_most_calories(input: &str) -> u32 {
    let elves_items = input.split("\n\n");

    let a = elves_items.map(|elve_items| elve_items.split("\n")
            .map(|item| item.parse::<u32>().unwrap_or(0))
            .sum())
        .max()
        .unwrap_or(0);

    a
}

fn main() {
    let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    println!("{}", get_most_calories(input))
}

#[cfg(test)]
mod tests {
    use crate::get_most_calories;

    #[test]
    fn tst_finds_elf_with_most_calories() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

        assert_eq!(get_most_calories(&input), 24000);
    }
}