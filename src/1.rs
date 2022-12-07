fn main() {
    let contents = std::fs::read_to_string("inputs/1.txt").expect("read input");

    let mut elves: Vec<i32> = contents
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|food| food.parse::<i32>().ok().unwrap_or(0))
                .sum()
        })
        .collect();

    elves.sort();

    println!("max: {}", elves.last().unwrap());

    let max_three: i32 = elves.iter().skip(elves.len() - 3).sum();

    println!("max three: {}", max_three);
}
