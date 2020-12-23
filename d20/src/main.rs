// Should work relatively fast. Code is a mess though.
// Can be optimized further to convert the data to Vec<char> and defining Index<(usize, usize)>.

use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

use crate::tile::*;
use crate::tiledata::TileData;

pub mod tile;
pub mod tiledata;

pub const NORTH: usize = 0;
pub const EAST: usize = 1;
pub const SOUTH: usize = 2;
pub const WEST: usize = 3;
pub const DIRECTIONS: [usize; 4] = [NORTH, EAST, SOUTH, WEST];

type Pos = (i32, i32);

fn read_data(filename: &str) -> String {
    std::fs::read_to_string(&filename).unwrap()
}

fn parse_data(data: &str) -> Vec<TileData> {
    data.split("\n\n")
        .filter_map(|d| {
            if !d.is_empty() {
                Some(TileData::from_str(d).unwrap())
            } else {
                None
            }
        })
        .collect()
}

fn get_neighbour_pos(pos: &Pos, direction: usize) -> Pos {
    match direction {
        NORTH => (pos.0, pos.1 - 1),
        EAST => (pos.0 + 1, pos.1),
        SOUTH => (pos.0, pos.1 + 1),
        WEST => (pos.0 - 1, pos.1),
        _ => panic!(),
    }
}

fn get_opposite_direction(direction: usize) -> usize {
    (direction + 2) % 4
}

/// This will rotate the tile to the opposite of target_direction, then match the flipping
fn find_correct_rotation_to_opposite(
    cur_direction: usize,
    target_direction: usize,
    tile: &Tile,
) -> Tile {
    let mut opposite = target_direction + 2 + 4;
    let rotations = (opposite - cur_direction) % 4;
    opposite = opposite % 4;
    let mut new_tile = tile.clockwise_rotated(rotations);
    if new_tile.edges[opposite] == tile.flipped_edges[cur_direction] {
        new_tile = if opposite == SOUTH || opposite == NORTH {
            new_tile.h_flipped()
        } else {
            new_tile.v_flipped()
        }
    }
    new_tile
}

fn process(tiles: &[Tile]) -> HashMap<Pos, Tile> {
    let mut processed_tiles: HashMap<Pos, Tile> = HashMap::new();
    let mut unprocessed_tiles: Vec<&Tile> = {
        let mut iter = tiles.iter();
        processed_tiles.insert((0, 0), iter.next().unwrap().clone());
        iter.collect()
    };

    while !unprocessed_tiles.is_empty() {
        let cur_tile = unprocessed_tiles.remove(0);
        let mut to_add: Option<(Pos, Tile)> = None;

        for (processed_pos, processed_tile) in processed_tiles.iter() {
            let matching_normal: Vec<(usize, usize)> = cur_tile
                .edges
                .iter()
                .enumerate()
                .filter_map(|(cur_direction, cur_edge)| {
                    processed_tile
                        .edges
                        .iter()
                        .position(|proc_edge| proc_edge == cur_edge)
                        .map(|proc_direction| (cur_direction, proc_direction))
                })
                .collect();

            let matching_flipped: Vec<(usize, usize)> = cur_tile
                .flipped_edges
                .iter()
                .enumerate()
                .filter_map(|(cur_direction, cur_edge)| {
                    processed_tile
                        .edges
                        .iter()
                        .position(|proc_edge| proc_edge == cur_edge)
                        .map(|proc_direction| (cur_direction, proc_direction))
                })
                .collect();

            if matching_normal.is_empty() && matching_flipped.is_empty() {
                continue;
            }
            if matching_normal.len() + matching_flipped.len() > 1 {
                panic!();
            }

            let flipped = matching_flipped.len() == 1;

            let (mut cur_direction, proc_direction) = if flipped {
                matching_flipped[0]
            } else {
                matching_normal[0]
            };

            let mut new_tile =
                find_correct_rotation_to_opposite(cur_direction, proc_direction, cur_tile);

            cur_direction = get_opposite_direction(proc_direction);
            if flipped {
                if cur_direction == EAST || cur_direction == WEST {
                    new_tile = new_tile.v_flipped()
                } else {
                    new_tile = new_tile.h_flipped()
                }
            }

            let tile_pos = get_neighbour_pos(processed_pos, proc_direction);

            let valid = DIRECTIONS.iter().all(|&direction| {
                if direction == cur_direction {
                    true
                } else {
                    let neigh_pos = get_neighbour_pos(&tile_pos, direction);
                    if let Some(neigh_tile) = processed_tiles.get(&neigh_pos) {
                        neigh_tile.edges[get_opposite_direction(direction)]
                            == new_tile.edges[direction]
                    } else {
                        true
                    }
                }
            });

            if valid {
                to_add = Some((tile_pos, new_tile));
                break;
            }
        }
        if let Some((pos, tile)) = to_add {
            processed_tiles.insert(pos, tile);
        } else {
            unprocessed_tiles.push(cur_tile);
        }
    }

    processed_tiles
}

fn find_corner_positions(fixed_tiles: &HashMap<Pos, Tile>) -> Vec<Pos> {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (0, 0, 0, 0);

    for &(x, y) in fixed_tiles.keys() {
        if x_min > x {
            x_min = x;
        } else if x_max < x {
            x_max = x;
        }
        if y_min > y {
            y_min = y;
        } else if y_max < y {
            y_max = y;
        }
    }

    vec![
        (x_min, y_max),
        (x_max, y_max),
        (x_max, y_min),
        (x_min, y_min),
    ]
}

const MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

fn main() {
    let tile_data = parse_data(&read_data("input"));
    let tiles: Vec<Tile> = tile_data.iter().map(|td| Tile::from(td)).collect();
    let fixed_tiles = process(&tiles);
    let corners = find_corner_positions(&fixed_tiles);

    let mul: usize = corners
        .iter()
        .map(|corner| fixed_tiles.get(corner).unwrap().tile_id)
        .fold(1, |acc, e| acc * e);

    println!("Result #1: {}", mul);

    let (x_min, y_min) = corners[3];
    let (x_max, y_max) = corners[1];
    let mut tile_map: HashMap<usize, TileData> =
        tile_data.into_iter().map(|t| (t.tile_id, t)).collect();

    for (_, tile) in fixed_tiles.iter() {
        let td = tile_map.get_mut(&tile.tile_id).unwrap();
        td.rotate_clockwise(tile.num_rotations);
        if tile.is_h_flipped {
            td.flip_horizontal();
        }
        if tile.is_v_flipped {
            td.flip_vertical();
        }

        for direction in &DIRECTIONS {
            td.remove_edge(*direction);
        }
    }
    let tile_size = tile_map.iter().next().unwrap().1.y_size;
    let num_tiles = tile_size * (x_max - x_min + 1) as usize;
    let mut str = vec![vec!['!'; num_tiles]; num_tiles];

    for row in y_min..=y_max {
        let str_row = tile_size * (row - y_min) as usize;
        for col in x_min..=x_max {
            let str_col = tile_size * (col - x_min) as usize;
            let t = tile_map
                .get(&fixed_tiles.get(&(col, row)).unwrap().tile_id)
                .unwrap();
            for y in 0..t.y_size {
                for x in 0..t.x_size {
                    str[str_row + y][str_col + x] = t.data[y][x];
                }
            }
        }
    }

    let r: Vec<String> = str
        .iter()
        .filter_map(|l| {
            let s = String::from_iter(l.iter());
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
        .collect();

    let monster_indexes: Vec<(usize, usize)> = MONSTER
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.chars().enumerate().filter_map(move |(col_idx, ch)| {
                if ch == '#' {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .collect();

    let s: String = "Tile: 1\n".to_owned() + &r.join("\n");
    let mut big_tile = TileData::from_str(&s).unwrap();

    let mut monster_count = 0;
    for _ in 0..4 {
        monster_count = search_monster(&monster_indexes, &mut big_tile);
        if monster_count > 0 {
            break;
        }
        big_tile.rotate_clockwise(1);
    }

    big_tile.rotate_clockwise(1);

    println!("Found {} monsters.", monster_count);

    let result_two: usize = big_tile
        .data
        .iter()
        .map(|l| {
            l.iter()
                .fold(0, |acc, &e| if e == '#' { acc + 1 } else { acc })
        })
        .sum();

    println!("Result #2: {}", result_two);
}

fn search_monster(monster_indexes: &Vec<(usize, usize)>, big_tile: &mut TileData) -> usize {
    let mut monster_count = 0;
    for row in 0..=big_tile.y_size - MONSTER.len() {
        for col in 0..=big_tile.x_size - MONSTER[0].len() {
            let found = monster_indexes
                .iter()
                .all(|(mrow, mcol)| big_tile.data[row + mrow][col + mcol] == '#');

            if found {
                monster_count += 1;

                for (mrow, mcol) in monster_indexes.iter() {
                    big_tile.data[row + mrow][col + mcol] = 'O';
                }
            }
        }
    }

    return monster_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_correct_rotation_to_opposite() {
        let cur_tile = Tile {
            tile_id: 0,
            edges: vec![1, 2, 3, 4],
            flipped_edges: vec![5, 6, 7, 8],
            num_rotations: 0,
            is_h_flipped: false,
            is_v_flipped: false,
        };

        let result = find_correct_rotation_to_opposite(EAST, SOUTH, &cur_tile);
        assert_eq!(3, result.num_rotations);

        assert_eq!(false, result.is_h_flipped);

        assert_eq!(false, result.is_v_flipped);

        assert_eq!(vec![2, 7, 4, 5], result.edges);
    }

    #[test]
    fn test_process() {
        let tile_data = parse_data(&read_data("testdata/test.txt"));
        let tiles: Vec<Tile> = tile_data.iter().map(|td| Tile::from(td)).collect();

        let result = process(&tiles);

        let corners = find_corner_positions(&result);

        let mul: usize = corners
            .iter()
            .map(|corner| result.get(corner).unwrap().tile_id)
            .fold(1, |acc, e| acc * e);

        assert_eq!(20899048083289, mul);
    }
}
