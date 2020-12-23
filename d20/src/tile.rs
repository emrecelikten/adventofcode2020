use crate::tiledata::TileData;
use crate::{DIRECTIONS, EAST, NORTH, SOUTH, WEST};

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub tile_id: usize,
    /// Contains a number representation of the edge treated as a binary number, left to right, top to down
    pub edges: Vec<usize>,
    /// Same representations, but the other way around (e.g. right to left)
    pub flipped_edges: Vec<usize>,
    pub num_rotations: usize,
    pub is_h_flipped: bool,
    pub is_v_flipped: bool,
}

impl Tile {
    pub fn calculate_edges(edge: &[&char]) -> (usize, usize) {
        let binary: String = edge
            .iter()
            .map(|&&ch| if ch == '#' { '1' } else { '0' })
            .collect();

        let lr = usize::from_str_radix(&binary, 2).unwrap();
        let rl = usize::from_str_radix(&binary.chars().rev().collect::<String>(), 2).unwrap();

        (lr, rl)
    }

    pub fn clockwise_rotated(&self, rotation_count: usize) -> Self {
        if rotation_count == 0 {
            return self.to_owned();
        }
        let mut new = self.clone();
        match rotation_count {
            1 => {
                new.edges[NORTH] = self.flipped_edges[WEST];
                new.edges[EAST] = self.edges[NORTH];
                new.edges[SOUTH] = self.flipped_edges[EAST];
                new.edges[WEST] = self.edges[SOUTH];
                new.flipped_edges[NORTH] = self.edges[WEST];
                new.flipped_edges[EAST] = self.flipped_edges[NORTH];
                new.flipped_edges[SOUTH] = self.edges[EAST];
                new.flipped_edges[WEST] = self.flipped_edges[SOUTH];
                new.num_rotations += 1;
            }
            2 => {
                new.edges[NORTH] = self.flipped_edges[SOUTH];
                new.edges[EAST] = self.flipped_edges[WEST];
                new.edges[SOUTH] = self.flipped_edges[NORTH];
                new.edges[WEST] = self.flipped_edges[EAST];
                new.flipped_edges[NORTH] = self.edges[SOUTH];
                new.flipped_edges[EAST] = self.edges[WEST];
                new.flipped_edges[SOUTH] = self.edges[NORTH];
                new.flipped_edges[WEST] = self.edges[EAST];
                new.num_rotations += 2;
            }
            3 => {
                new.edges[NORTH] = self.edges[EAST];
                new.edges[EAST] = self.flipped_edges[SOUTH];
                new.edges[SOUTH] = self.edges[WEST];
                new.edges[WEST] = self.flipped_edges[NORTH];
                new.flipped_edges[NORTH] = self.flipped_edges[EAST];
                new.flipped_edges[EAST] = self.edges[SOUTH];
                new.flipped_edges[SOUTH] = self.flipped_edges[WEST];
                new.flipped_edges[WEST] = self.edges[NORTH];
                new.num_rotations += 3;
            }
            _ => {
                panic!("Invalid parameter!");
            }
        }

        new
    }

    pub fn h_flipped(&self) -> Self {
        let mut new = self.clone();
        new.edges[NORTH] = self.flipped_edges[NORTH];
        new.edges[EAST] = self.edges[WEST];
        new.edges[SOUTH] = self.flipped_edges[SOUTH];
        new.edges[WEST] = self.edges[EAST];

        new.flipped_edges[NORTH] = self.edges[NORTH];
        new.flipped_edges[EAST] = self.flipped_edges[WEST];
        new.flipped_edges[SOUTH] = self.edges[SOUTH];
        new.flipped_edges[WEST] = self.flipped_edges[EAST];

        new.is_h_flipped = !new.is_h_flipped;
        new
    }

    pub fn v_flipped(&self) -> Self {
        let mut new = self.clone();
        new.edges[NORTH] = self.edges[SOUTH];
        new.edges[EAST] = self.flipped_edges[EAST];
        new.edges[SOUTH] = self.edges[NORTH];
        new.edges[WEST] = self.flipped_edges[WEST];

        new.flipped_edges[NORTH] = self.flipped_edges[SOUTH];
        new.flipped_edges[EAST] = self.edges[EAST];
        new.flipped_edges[SOUTH] = self.flipped_edges[NORTH];
        new.flipped_edges[WEST] = self.edges[WEST];

        new.is_v_flipped = !new.is_v_flipped;
        new
    }
}

impl From<&TileData> for Tile {
    fn from(data: &TileData) -> Self {
        let mut tile = Tile {
            tile_id: data.tile_id,
            edges: Vec::new(),
            flipped_edges: Vec::new(),
            num_rotations: 0,
            is_h_flipped: false,
            is_v_flipped: false,
        };

        for direction in &DIRECTIONS {
            let (lr, rl) = Tile::calculate_edges(&data.get_edge(*direction));
            tile.edges.push(lr);
            tile.flipped_edges.push(rl);
        }
        tile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const TEST_2311: &'static str = r"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

    #[test]
    fn test_tile_new() {
        let tile = Tile::from(&TileData::from_str(&TEST_2311).unwrap());

        assert_eq!(2311, tile.tile_id);

        assert_eq!(210, tile.edges[0]);

        assert_eq!(89, tile.edges[1]);

        assert_eq!(616, tile.flipped_edges[1]);
    }

    #[test]
    fn test_right_rotated() {
        let tile = Tile::from(&TileData::from_str(&TEST_2311).unwrap());

        assert_eq!(210, tile.edges[0]);

        assert_eq!(tile.flipped_edges[3], tile.clockwise_rotated(1).edges[0]);

        assert_eq!(
            tile.edges,
            tile.clockwise_rotated(1)
                .clockwise_rotated(1)
                .clockwise_rotated(1)
                .clockwise_rotated(1)
                .edges
        );

        assert_eq!(
            tile.clockwise_rotated(3).edges,
            tile.clockwise_rotated(1).clockwise_rotated(2).edges
        );
    }

    #[test]
    fn test_flip() {
        let tile = Tile::from(&TileData::from_str(&TEST_2311).unwrap());

        assert_eq!(tile.h_flipped().h_flipped(), tile);

        assert_eq!(tile.v_flipped().v_flipped(), tile);
    }
}
