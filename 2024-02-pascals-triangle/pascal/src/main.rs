use itertools::Itertools as _;

// copied from Rosetta Code
// https://rosettacode.org/wiki/Pascal's_triangle#Rust
fn pascal(rows: u64) {
    for row in 0..rows {
        let mut col = 1;

        for _ in 1..=2 * (rows - 1 - row) {
            print!(" ");
        }

        for position in 0..=row {
            print!("{col:3} ");

            col = col * (row - position) / (position + 1);
        }

        println!()
    }
}

/// Using a vec.
mod built_with_vec {
    pub fn pascal(rows: u64) {
        if rows == 0 {
            return;
        }

        let mut row = vec![1];
        println!("{row:?}");

        for _ in 1..rows {
            row.insert(0, 0);
            row.push(0);
            let mut next_row = Vec::with_capacity(row.len());

            let pairs = row.windows(2);
            for pair in pairs {
                match pair {
                    [left, right] => next_row.push(left + right),
                    _ => unreachable!(),
                }
            }

            println!("{next_row:?}");
            row = next_row;
        }
    }
}

/// Extra credit. This implementation is curtesy of robjtede.
mod as_iterator {
    use itertools::Itertools as _;

    #[derive(Debug)]
    pub struct PascalsTriangle {
        row: u32,
        prev: Vec<i32>,
    }

    impl PascalsTriangle {
        pub fn new() -> Self {
            Self {
                row: 0,
                prev: vec![1],
            }
        }
    }

    impl Iterator for PascalsTriangle {
        type Item = Vec<i32>;

        fn next(&mut self) -> Option<Self::Item> {
            self.row += 1;

            if self.row == 1 {
                return Some(self.prev.clone());
            }

            self.prev.insert(0, 0);
            self.prev.push(0);

            self.prev = self
                .prev
                .iter()
                .tuple_windows()
                .map(|(a, b)| a + b)
                .collect();

            Some(self.prev.clone())
        }
    }
}

fn main() {
    pascal(7);

    built_with_vec::pascal(7);

    let pas = as_iterator::PascalsTriangle::new();
    for row in pas.take(7) {
        println!("{}", row.iter().join(", "));
    }
}
