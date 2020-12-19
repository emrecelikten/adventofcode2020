extern crate lazy_static;

use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;

use lazy_static::lazy_static;

type Position = (i32, i32, i32, i32);
lazy_static! {
    static ref OFFSETS: Vec<Position> = {
        let mut position_vec = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    position_vec.push((x, y, z, 0));
                }
            }
        }
        position_vec
    };
    static ref OFFSETS4D: Vec<Position> = {
        let mut position_vec = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }
                        position_vec.push((x, y, z, w));
                    }
                }
            }
        }
        position_vec
    };
}

fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

#[derive(Clone, PartialEq)]
struct SparseBoolMatrix {
    underlying: HashSet<Position>,
    size: usize,
}

impl SparseBoolMatrix {
    fn new() -> Self {
        SparseBoolMatrix {
            underlying: Default::default(),
            size: 0,
        }
    }

    fn contains(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        self.underlying.contains(&(x, y, z, w))
    }

    fn set(&mut self, x: i32, y: i32, z: i32, w: i32) {
        self.underlying.insert((x, y, z, w));
    }

    fn num_neighbours(&self, x: i32, y: i32, z: i32, w: i32, offsets: &Vec<Position>) -> u8 {
        let mut result = 0;
        for (cx, cy, cz, cw) in offsets.iter() {
            if self.underlying.contains(&(x + cx, y + cy, z + cz, w + cw)) {
                result += 1;
            }
        }
        result
    }
}

impl fmt::Debug for SparseBoolMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Performance is probably not great here
        dbg!(self.size);
        let mut m = vec![vec![vec![vec!['.'; self.size]; self.size]; self.size]; self.size];

        let offset = self.size as i32 / 2 - 1;

        // Let's do 3d only for now
        for &(x, y, z, w) in self.underlying.iter() {
            m[(w + offset) as usize][(z + offset) as usize][(y + offset) as usize]
                [(x + offset) as usize] = '#';
        }

        let mut str = "\n".to_owned();
        for zvec in m[(2 + offset) as usize - 1].iter() {
            for yvec in zvec {
                str += &String::from_iter(yvec.iter());
                str += "\n";
            }
            str += "\n\n";
        }

        write!(f, "{}", str)
    }
}

fn parse_data(data: &str) -> SparseBoolMatrix {
    let mut result: SparseBoolMatrix = SparseBoolMatrix::new();
    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                result.set(col as i32, row as i32, 0, 0);
            }
        }
    }
    result.size = data.lines().nth(0).unwrap().len(); // Assuming square
    result
}

fn iterate(matrix: &SparseBoolMatrix, num_iter: usize, offsets: &Vec<Position>) -> usize {
    let mut cur_matrix = matrix.clone();
    let mut new_matrix: SparseBoolMatrix = SparseBoolMatrix::new();
    for _ in 0..num_iter {
        for (x, y, z, w) in cur_matrix.underlying.iter() {
            for (ox, oy, oz, ow) in offsets.iter() {
                let cx = x + ox;
                let cy = y + oy;
                let cz = z + oz;
                let cw = w + ow;

                if new_matrix.contains(cx, cy, cz, cw) {
                    continue;
                }

                let val = cur_matrix.contains(cx, cy, cz, cw);
                let num_neighbors = cur_matrix.num_neighbours(cx, cy, cz, cw, offsets);

                if !val && num_neighbors == 3 {
                    new_matrix.set(cx, cy, cz, cw);
                } else if val && num_neighbors >= 2 && num_neighbors <= 3 {
                    new_matrix.set(cx, cy, cz, cw);
                }
            }
        }
        new_matrix.size = cur_matrix.size + 2;
        cur_matrix = new_matrix;
        new_matrix = SparseBoolMatrix::new();
    }

    cur_matrix.underlying.len()
}

fn main() {
    let data = parse_data(&read_data("input"));
    let result_one = iterate(&data, 6, &OFFSETS);
    println!("Result #1: {}", result_one);

    let result_two = iterate(&data, 6, &OFFSETS4D);
    println!("Result #2: {}", result_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = r".#.
..#
###";

    #[test]
    fn test_sparse() {
        let mut matrix = SparseBoolMatrix::new();
        matrix.set(1, 5, 2, 0);
        matrix.set(1, 5, 3, 0);
        matrix.set(2, 5, 2, 0);
        assert_eq!(matrix.contains(1, 5, 2, 0), true);
        assert_eq!(matrix.contains(1, 5, 3, 0), true);
        assert_eq!(matrix.contains(2, 5, 2, 0), true);
        assert_eq!(matrix.contains(2, 5, 3, 0), false);
    }

    #[test]
    fn test_parse() {
        let matrix = parse_data(&TEST_DATA);
        assert_eq!(matrix.contains(1, 0, 0, 0), true);
        assert_eq!(matrix.contains(0, 0, 0, 0), false);
        assert_eq!(matrix.contains(2, 2, 0, 0), true);

        assert_eq!(matrix.num_neighbours(1, 2, 0, 0, &OFFSETS), 3);
        assert_eq!(matrix.num_neighbours(2, 2, 0, 0, &OFFSETS), 2);
    }

    #[test]
    fn test_part_one() {
        let matrix = parse_data(&TEST_DATA);

        let result = iterate(&matrix, 6, &OFFSETS);
        assert_eq!(result, 112);
    }

    #[test]
    fn test_part_two() {
        let matrix = parse_data(&TEST_DATA);

        let result = iterate(&matrix, 6, &OFFSETS4D);
        assert_eq!(result, 848);
    }
}
