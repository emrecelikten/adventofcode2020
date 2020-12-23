use std::str::FromStr;

use crate::{EAST, NORTH, SOUTH, WEST};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;
use std::num::ParseIntError;

pub struct TileData {
    pub tile_id: usize,
    pub x_size: usize,
    pub y_size: usize,
    pub data: Vec<Vec<char>>,
}

impl TileData {
    pub fn get_edge(&self, direction: usize) -> Vec<&char> {
        match direction {
            NORTH => self.data[0].iter().collect(),
            EAST => self.data.iter().map(|l| &l[self.x_size - 1]).collect(),
            SOUTH => self.data[self.y_size - 1].iter().collect(),
            WEST => self.data.iter().map(|l| &l[0]).collect(),
            _ => panic!("Invalid direction!"),
        }
    }

    pub fn remove_edge(&mut self, direction: usize) {
        match direction {
            NORTH => {
                self.y_size -= 1;
                self.data.remove(0);
            }
            EAST => {
                let s = self.x_size - 1;
                self.x_size = s;

                self.data.iter_mut().for_each(|l| {
                    l.remove(s);
                })
            }
            SOUTH => {
                self.y_size -= 1;
                self.data.remove(self.y_size);
            }
            WEST => {
                self.x_size -= 1;
                self.data.iter_mut().for_each(|l| {
                    l.remove(0);
                })
            }
            _ => panic!("Invalid direction!"),
        }
    }

    pub fn rotate_clockwise(&mut self, times: usize) {
        if times % 4 == 0 {
            return;
        }
        match times {
            1 => {
                let mut new_data = vec![vec!['.'; self.y_size]; self.x_size];
                for row in 0..self.y_size {
                    for col in 0..self.x_size {
                        new_data[col][self.y_size - 1 - row] = self.data[row][col];
                    }
                }
                std::mem::swap(&mut self.x_size, &mut self.y_size);
                self.data = new_data;
            }
            2 => {
                let mut new_data = vec![vec!['.'; self.x_size]; self.y_size];
                for row in 0..self.y_size {
                    for col in 0..self.x_size {
                        new_data[self.y_size - 1 - row][self.x_size - 1 - col] =
                            self.data[row][col];
                    }
                }
                self.data = new_data;
            }
            3 => {
                let mut new_data = vec![vec!['.'; self.y_size]; self.x_size];
                for row in 0..self.y_size {
                    for col in 0..self.x_size {
                        new_data[self.x_size - 1 - col][row] = self.data[row][col];
                    }
                }
                std::mem::swap(&mut self.x_size, &mut self.y_size);
                self.data = new_data;
            }
            _ => {}
        }
    }

    pub fn flip_horizontal(&mut self) {
        let mut new_data = vec![vec!['.'; self.x_size]; self.y_size];
        for row in 0..self.y_size {
            for col in 0..self.x_size {
                new_data[row][self.x_size - 1 - col] = self.data[row][col];
            }
        }
        self.data = new_data;
    }

    pub fn flip_vertical(&mut self) {
        let mut new_data = vec![Vec::new(); self.y_size];
        for row in 0..self.y_size {
            new_data[self.y_size - 1 - row] = self.data[row].clone();
        }
        self.data = new_data;
    }
}

impl FromStr for TileData {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let tile_id = lines
            .next()
            .unwrap()
            .chars()
            .filter(|ch| ch.is_numeric())
            .collect::<String>()
            .parse()?;
        let data: Vec<Vec<char>> = lines.map(|x| x.chars().collect()).collect();
        let x_size = data[0].len();
        let y_size = data.len();

        Ok(TileData {
            tile_id,
            x_size,
            y_size,
            data,
        })
    }
}

impl Debug for TileData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str = "\n".to_owned();

        for line in self.data.iter() {
            str.push_str(&String::from_iter(line.iter()));
            str.push_str("\n");
        }

        f.write_str(&str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TILE: &'static str = r"Tile 1:
123
456";

    #[test]
    fn test_rotate() {
        let mut tile = TileData::from_str(&TILE).unwrap();

        tile.rotate_clockwise(1);
        assert_eq!(vec!['6', '3'], tile.data[2]);

        tile.rotate_clockwise(2);
        assert_eq!(vec!['3', '6'], tile.data[0]);

        tile.rotate_clockwise(3);
        assert_eq!(vec!['6', '5', '4'], tile.data[0]);
    }

    #[test]
    fn test_flip_horizontal() {
        let mut tile = TileData::from_str(&TILE).unwrap();

        tile.flip_horizontal();

        assert_eq!(vec!['3', '2', '1'], tile.data[0]);
    }
}
