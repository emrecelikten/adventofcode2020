use std::convert::TryFrom;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::string::ParseError;

const POSITIONS: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    data: Vec<char>,
    ysize: usize,
    xsize: usize,
}

impl Index<(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1 * self.xsize + index.0]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1 * self.xsize + index.0]
    }
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xsize = s.find('\n').unwrap();
        let data: Vec<char> = s.chars()
            .filter(|&x| x != '\n')
            .collect();
        let ysize = data.len() / xsize;

        Ok(Grid {
            data,
            ysize,
            xsize,
        })
    }
}

fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn count_adjacent_occupied(ux: usize, uy: usize, data: &Grid) -> Option<usize> {
    let x = isize::try_from(ux).unwrap();
    let y = isize::try_from(uy).unwrap();
    let ysize = isize::try_from(data.ysize).unwrap();
    let xsize = isize::try_from(data.xsize).unwrap();
    if x < 0 || x >= xsize { return None; }
    if y < 0 || y >= ysize { return None; }

    let mut count = 0;

    for (xoffset, yoffset) in &POSITIONS {
        let newx = x + xoffset;
        if newx < 0 || newx >= xsize { continue; }
        let newy = y + yoffset;
        if newy < 0 || newy >= ysize { continue; }

        if data[(usize::try_from(newx).unwrap(), usize::try_from(newy).unwrap())] == '#' { count += 1; }
    }

    Some(count)
}

fn count_first(ux: usize, uy: usize, data: &Grid) -> Option<usize> {
    let x = isize::try_from(ux).unwrap();
    let y = isize::try_from(uy).unwrap();
    let ysize = isize::try_from(data.ysize).unwrap();
    let xsize = isize::try_from(data.xsize).unwrap();
    if x < 0 || x >= xsize { return None; }
    if y < 0 || y >= ysize { return None; }

    let mut occupied_count = 0;

    for (xoffset, yoffset) in &POSITIONS {
        let mut newx = x;
        let mut newy = y;

        loop {
            newx += xoffset;
            if newx < 0 || newx >= xsize { break; }
            newy += yoffset;
            if newy < 0 || newy >= ysize { break; }

            let ch = data[(usize::try_from(newx).unwrap(), usize::try_from(newy).unwrap())];
            if ch != '.' {
                if ch == '#' { occupied_count += 1; }
                break;
            }
        }
    }

    Some(occupied_count)
}

fn iterate_one(data: &Grid) -> Grid {
    let mut result: Grid = data.clone();
    for y in 0..data.ysize {
        for x in 0..data.xsize {
            let ch = data[(x, y)];
            if ch != '.' {
                if let Some(occupied) = count_adjacent_occupied(x, y, &data) {
                    if ch == '#' && occupied >= 4 {
                        result[(x, y)] = 'L';
                    } else if ch == 'L' && occupied == 0 {
                        result[(x, y)] = '#';
                    }
                }
            }
        }
    }
    result
}

fn iterate_two(data: &Grid) -> Grid {
    let mut result: Grid = data.clone();
    for y in 0..data.ysize {
        for x in 0..data.xsize {
            let ch = data[(x, y)];
            if ch != '.' {
                if let Some(occupied) = count_first(x, y, &data) {
                    if ch == '#' && occupied >= 5 {
                        result[(x, y)] = 'L';
                    } else if ch == 'L' && occupied == 0 {
                        result[(x, y)] = '#';
                    }
                }
            }
        }
    }
    result
}

fn main() {
    let data = read_data("input");
    let initial_grid = Grid::from_str(&data).unwrap();
    let mut grid = initial_grid.clone();

    loop {
        let new_grid = iterate_one(&grid);
        if grid == new_grid { break; } else { grid = new_grid; }
    }

    let part_one_count = grid.data.iter().filter(|&&x| x == '#').count();
    println!("Number of occupied seats: {}", part_one_count);

    grid = initial_grid;
    loop {
        let new_grid = iterate_two(&grid);
        if grid == new_grid { break; } else { grid = new_grid; }
    }

    let part_two_count = grid.data.iter().filter(|&&x| x == '#').count();
    println!("Number of occupied seats: {}", part_two_count);
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    const ADJACENT_OCCUPIED_TEST_DATA: &'static str = r"L.LL
#.#.
.L.#
..L#";

    #[test]
    fn test_count_adjacent_occupied() {
        let grid: Grid = Grid::from_str(ADJACENT_OCCUPIED_TEST_DATA).unwrap();
        assert_eq!(grid.xsize, 4);
        assert_eq!(grid.ysize, 4);

        assert_eq!(count_adjacent_occupied(0, 0, &grid), Some(1));
        assert_eq!(count_adjacent_occupied(3, 3, &grid), Some(1));
        assert_eq!(count_adjacent_occupied(2, 2, &grid), Some(3));
    }

    #[test]
    fn test_iterate_one() {
        let initial_str = std::fs::read_to_string(Path::new("testdata/grid_iter_0.txt")).unwrap();
        let initial_grid = Grid::from_str(&initial_str).unwrap();
        assert_eq!(initial_grid.xsize, 10);
        assert_eq!(initial_grid.ysize, 10);

        let iter1_str = std::fs::read_to_string(Path::new("testdata/grid_iter_1.txt")).unwrap();
        let iter1_grid = iterate_one(&initial_grid);
        let expected_iter1_grid = Grid::from_str(&iter1_str).unwrap();
        assert_eq!(iter1_grid, expected_iter1_grid);

        let iter2_str = std::fs::read_to_string(Path::new("testdata/grid_iter_2.txt")).unwrap();
        let iter2_grid = iterate_one(&iter1_grid);
        let expected_iter2_grid = Grid::from_str(&iter2_str).unwrap();
        assert_eq!(iter2_grid, expected_iter2_grid);

        let iter3_str = std::fs::read_to_string(Path::new("testdata/grid_iter_3.txt")).unwrap();
        let iter3_grid = iterate_one(&iter2_grid);
        let expected_iter3_grid = Grid::from_str(&iter3_str).unwrap();
        assert_eq!(iter3_grid, expected_iter3_grid);
    }

    #[test]
    fn test_count_first() {
        let test1_str = std::fs::read_to_string(Path::new("testdata/first_visible_1.txt")).unwrap();
        let test1_grid = Grid::from_str(&test1_str).unwrap();
        assert_eq!(count_first(3, 4, &test1_grid), Some(8));

        let test2_str = std::fs::read_to_string(Path::new("testdata/first_visible_2.txt")).unwrap();
        let test2_grid = Grid::from_str(&test2_str).unwrap();
        assert_eq!(count_first(1, 1, &test2_grid), Some(0));

        let test3_str = std::fs::read_to_string(Path::new("testdata/first_visible_3.txt")).unwrap();
        let test3_grid = Grid::from_str(&test3_str).unwrap();
        assert_eq!(count_first(3, 3, &test3_grid), Some(0));
    }

    #[test]
    fn test_iterate_two() {
        let initial_str = std::fs::read_to_string(Path::new("testdata/grid_iter_0.txt")).unwrap();
        let initial_grid = Grid::from_str(&initial_str).unwrap();
        assert_eq!(initial_grid.xsize, 10);
        assert_eq!(initial_grid.ysize, 10);

        let iter1_str = std::fs::read_to_string(Path::new("testdata/grid_iter_1.txt")).unwrap();
        let iter1_grid = iterate_two(&initial_grid);
        let expected_iter1_grid = Grid::from_str(&iter1_str).unwrap();
        assert_eq!(iter1_grid, expected_iter1_grid);

        let iter2_str = std::fs::read_to_string(Path::new("testdata/grid_iter_v2_2.txt")).unwrap();
        let iter2_grid = iterate_two(&iter1_grid);
        let expected_iter2_grid = Grid::from_str(&iter2_str).unwrap();
        assert_eq!(iter2_grid, expected_iter2_grid);

        let iter3_str = std::fs::read_to_string(Path::new("testdata/grid_iter_v2_3.txt")).unwrap();
        let iter3_grid = iterate_two(&iter2_grid);
        let expected_iter3_grid = Grid::from_str(&iter3_str).unwrap();
        assert_eq!(iter3_grid, expected_iter3_grid);

        let iter4_str = std::fs::read_to_string(Path::new("testdata/grid_iter_v2_4.txt")).unwrap();
        let iter4_grid = iterate_two(&iter3_grid);
        let expected_iter4_grid = Grid::from_str(&iter4_str).unwrap();
        assert_eq!(iter4_grid, expected_iter4_grid);
    }
}