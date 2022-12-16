#![feature(is_sorted)]

struct Matrix<T> {
    rows: usize,
    columns: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    fn row(&self, r: usize) -> RowIter<T> {
        RowIter {
            row: r,
            col_len: self.columns,
            data: &self.data,
        }
    }
    fn column(&self, c: usize) -> ColumnIter<T> {
        ColumnIter {
            row_len: self.rows,
            column: c,
            data: &self.data,
        }
    }
}

struct RowIter<'a, T> {
    row: usize,
    col_len: usize,
    data: &'a [T],
}
struct ColumnIter<'a, T> {
    row_len: usize,
    column: usize,
    data: &'a [T],
}

impl<'a, T> std::iter::Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> std::iter::Iterator for ColumnIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

struct MatrixBuilder<T> {
    rows: usize,
    columns: usize,
    data: Vec<T>,
}

impl<T: Clone> MatrixBuilder<T> {
    fn new() -> Self {
        MatrixBuilder {
            rows: 0,
            columns: 0,
            data: vec![],
        }
    }

    fn set_rows(mut self, r: usize) -> Self {
        self.rows = r;

        self
    }

    fn set_columns(mut self, c: usize) -> Self {
        self.columns = c;

        self
    }

    fn add_data(mut self, data: &[T]) -> Self {
        self.data.extend(data.iter().cloned());

        self
    }

    fn build(self) -> Matrix<T> {
        if self.rows * self.columns != self.data.len() {
            panic!("Cannot create Matrix");
        }

        Matrix {
            rows: self.rows,
            columns: self.columns,
            data: self.data,
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/8.txt").expect("read input");

    let mut matrix = Vec::<Vec<u8>>::new();

    let mut mb = MatrixBuilder::new();

    let mut rows = 0;
    for l in contents.lines() {
        rows += 1;
        mb = mb.set_columns(l.len());
        mb = mb.add_data(l.as_bytes());
    }
    mb = mb.set_rows(rows);

    let matrix = mb.build();

    let mut visible = 0;

    // for line in &matrix {
    //     for i in 0..line.len() {
    //         if line.iter().take(i + 1).is_sorted() {
    //             visible += 1;
    //         }
    //     }
    //     for i in 0..line.len() {
    //         if line.iter().rev().take(i + 1).is_sorted() {
    //             visible += 1;
    //         }
    //     }
    // }

    // println!("visible {visible}");

    // for i in 1..matrix[0].len() - 1 {
    //     visible += 1;
    //     for j in 1.. {
    //         if matrix[j][i] < matrix[j + 1][i] {
    //             visible += 1;
    //         }
    //         break;
    //     }

    //     visible += 1;
    //     for j in (0..=matrix.len() - 1).rev() {
    //         if matrix[j - 1][i] > matrix[j][i] {
    //             visible += 1;
    //         }
    //         break;
    //     }
    // }

    // println!("visible {visible}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() {
        let m = MatrixBuilder::new()
            .set_columns(3)
            .set_rows(3)
            .add_data(&[1, 2, 3])
            .add_data(&[4, 5, 6])
            .add_data(&[7, 8, 9])
            .build();

        let it = m.row(1);

        assert_eq!(it.next(), Some(&4));
        assert_eq!(it.next(), Some(&5));
        assert_eq!(it.next(), Some(&6));
        assert_eq!(it.next(), None);

        let it = m.column(1);

        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&5));
        assert_eq!(it.next(), Some(&8));
        assert_eq!(it.next(), None);
    }
}
