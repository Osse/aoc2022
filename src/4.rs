use std::collections::HashSet;

fn parse_range(s: &str) -> Option<HashSet<i32>> {
    let (from, to) = s.split_once('-')?;
    let (from, to) = (from.parse::<i32>().ok()?, to.parse::<i32>().ok()?);

    Some((from..=to).collect())
}

fn main() {
    let contents = std::fs::read_to_string("inputs/4.txt").expect("read input");

    let ranges: Vec<(HashSet<i32>, HashSet<i32>)> = contents
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(',').expect("split once at comma");

            let left = parse_range(left).expect("found range");
            let right = parse_range(right).expect("found range");

            (left, right)
        })
        .collect();

    println!(
        "contained: {}",
        ranges
            .iter()
            .filter(|(l, r)| l.is_subset(r) || l.is_superset(r))
            .count()
    );

    println!(
        "overlaps: {}",
        ranges
            .iter()
            .filter(|(l, r)| l.intersection(r).count() > 0)
            .count()
    );
}
