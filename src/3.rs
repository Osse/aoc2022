use std::collections::HashSet;

fn priority(c: u8) -> u32 {
    let p = match c {
        b'a'..=b'z' => c - 0x60,
        b'A'..=b'Z' => c - 0x40 + 26,
        _ => panic!("lol char"),
    };

    p as u32
}

fn main() {
    let contents = std::fs::read_to_string("inputs/3.txt").expect("read input");

    let total_priority: u32 = contents
        .lines()
        .map(|l| {
            let (left, right) = l.split_at(l.len() / 2);

            dbg!(&left);
            dbg!(&right);

            let left = HashSet::<u8>::from_iter(left.bytes());
            let right = HashSet::<u8>::from_iter(right.bytes());

            let only: Vec<u8> = left.intersection(&right).cloned().collect();

            if only.len() != 1 {
                dbg!("wtf");
            }

            let only = only.first().cloned().unwrap();

            dbg!(&only);

            priority(only)
        })
        .sum();

    let lines: Vec<&str> = contents.lines().collect();

    let total_priority: u32 = lines
        .chunks(3)
        .map(|w| {
            let one = w[0].bytes().collect::<HashSet<_>>();
            let two = w[1].bytes().collect::<HashSet<_>>();
            let three = w[2].bytes().collect::<HashSet<_>>();

            dbg!(&one);
            dbg!(&two);
            dbg!(&three);

            let kek = one.intersection(&two).cloned().collect::<HashSet<_>>();
            let only = kek.intersection(&three).collect::<Vec<_>>();

            let only = only.first().cloned().unwrap();

            dbg!(&only);

            priority(*only)
        })
        .sum();

    println!("pri: {}", total_priority);
}
