#[derive(Clone, Debug)]
struct Stack(Vec<char>);

impl Stack {
    fn new() -> Self {
        Self(vec![])
    }

    fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    fn push(&mut self, c: char) {
        self.0.push(c)
    }

    fn top(&self) -> Option<&char> {
        self.0.last()
    }

    fn pop_many(&mut self, n: usize) -> Vec<char> {
        self.0.drain((self.0.len() - n)..).collect()
    }

    fn push_many(&mut self, v: Vec<char>) {
        self.0.extend(v.into_iter());
    }
}

#[derive(Debug)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn from(diagram: &str) -> Self {
        let mut riter = diagram.lines().rev();

        let numbers_line = riter.next().unwrap();

        let indices: Vec<usize> = numbers_line
            .match_indices(|c: char| c.is_ascii_digit())
            .map(|(idx, _s)| idx)
            .collect();

        let mut stacks = vec![Stack::new(); indices.len()];

        for line in riter {
            for (stack_idx, char_idx) in indices.iter().enumerate() {
                if let Some(c) = line.chars().nth(*char_idx) {
                    if c.is_uppercase() {
                        stacks[stack_idx].push(c)
                    }
                }
            }
        }

        Self(stacks)
    }

    fn rearrange(&mut self, from: usize, to: usize) {
        let c = self.0[from - 1].pop().unwrap();
        self.0[to - 1].push(c);
    }

    fn rearrange_9001(&mut self, crates: usize, from: usize, to: usize) {
        let c = self.0[from - 1].pop_many(crates);
        self.0[to - 1].push_many(c);
    }

    fn tops(&self) -> String {
        let mut s = String::new();
        for v in &self.0 {
            s.push(v.top().unwrap_or(&' ').clone());
        }

        s
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/5.txt").expect("read input");

    let mut iter = contents.split("\n\n");

    let diagram = iter.next().unwrap();
    let mut stacks = Stacks::from(diagram);

    let instructions = iter.next().unwrap();

    let re = regex::Regex::new(r#"^move (\d+) from (\d+) to (\d+)$"#).unwrap();

    for instruction in instructions.split_terminator("\n") {
        if let Some(captures) = re.captures(instruction) {
            let n = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

            // for _i in 0..n {
            stacks.rearrange_9001(n, from, to);
            // }
        }
    }

    println!("tops: {}", stacks.tops());
}
