#![feature(is_sorted)]
fn main() {
    let contents = std::fs::read_to_string("inputs/8.txt").expect("read input");

    let mut matrix = Vec::<Vec<u8>>::new();

    for l in contents.lines() {
        matrix.push(l.as_bytes().to_owned());
    }

    let mut visible = 0;

    for line in &matrix {
        for i in 0..line.len() {
            if line.iter().take(i + 1).is_sorted() {
                visible += 1;
            }
        }
        for i in 0..line.len() {
            if line.iter().rev().take(i + 1).is_sorted() {
                visible += 1;
            }
        }
    }

    println!("visible {visible}");

    for i in 1..matrix[0].len() - 1 {
        visible += 1;
        for j in 1.. {
            if matrix[j][i] < matrix[j + 1][i] {
                visible += 1;
            }
            break;
        }

        visible += 1;
        for j in (0..=matrix.len() - 1).rev() {
            if matrix[j - 1][i] > matrix[j][i] {
                visible += 1;
            }
            break;
        }
    }

    println!("visible {visible}");
}
