use std::collections::HashSet;
fn main() {
    let contents = std::fs::read_to_string("inputs/6.txt").expect("read input");

    let unique_len = 14;

    if let Some(pos) = contents
        .as_bytes()
        .windows(unique_len)
        .position(|w| w.iter().copied().collect::<HashSet<u8>>().len() == unique_len)
    {
        println!("position: {}", pos);
        println!("characters: {}", pos + unique_len);
    }
}
