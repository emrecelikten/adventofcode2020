use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Hexagon {
    is_black: bool,
}

type Pos = (i32, i32);
type HexagonMap = HashMap<Pos, Hexagon>;

fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn parse_line(line: &str) -> Pos {
    let (mut x, mut y) = (0, 0);
    let mut chars = line.chars();

    while let Some(ch) = chars.next() {
        match ch {
            'e' => {
                x += 1;
                continue;
            }
            'w' => {
                x -= 1;
                continue;
            }
            'n' => {
                y -= 1;
            }
            's' => {
                y += 1;
            }
            _ => panic!(),
        }
        let ch2 = chars.next().unwrap();
        if ch == 'n' && ch2 == 'e' {
            x += 1;
        } else if ch == 's' && ch2 == 'w' {
            x -= 1;
        }
    }

    (x, y)
}

fn construct_map(lines: &Vec<&str>) -> HexagonMap {
    let iter = lines.iter().cloned().map(parse_line);

    let mut map = HexagonMap::new();
    for pos in iter {
        if map.contains_key(&pos) {
            let v = map.get_mut(&pos).unwrap();
            v.is_black = !v.is_black;
        } else {
            map.insert(pos, Hexagon { is_black: true });
        }
    }

    map
}

fn count_black_tiles(map: &HexagonMap) -> usize {
    map.values().filter(|hex| hex.is_black).count()
}

const POSITIONS: [Pos; 6] = [(0, -1), (1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0)];

fn count_black_neighbours(map: &HexagonMap, current_tile_pos: &Pos) -> usize {
    let (cur_x, cur_y) = current_tile_pos;
    POSITIONS.iter().fold(0, |acc, (x, y)| {
        let new_pos = (cur_x + x, cur_y + y);
        if map.contains_key(&new_pos) && map.get(&new_pos).unwrap().is_black {
            acc + 1
        } else {
            acc
        }
    })
}

fn iterate_day(map: &HexagonMap) -> HexagonMap {
    let mut new_map = HexagonMap::new();

    for pos in map.keys() {
        let iter = POSITIONS
            .iter()
            .map(|(x, y)| (pos.0 + x, pos.1 + y))
            .chain(std::iter::once(*pos));

        for new_pos in iter {
            if new_map.contains_key(&new_pos) {
                continue;
            }
            let mut new_hex = map
                .get(&new_pos)
                .cloned()
                .unwrap_or(Hexagon { is_black: false });
            let num_neigh = count_black_neighbours(&map, &new_pos);
            // dbg!(&pos, &new_pos, new_hex.is_black, &num_neigh);
            if (new_hex.is_black && (num_neigh == 0 || num_neigh > 2))
                || (!new_hex.is_black && num_neigh == 2)
            {
                new_hex.is_black = !new_hex.is_black;
                // dbg!("FLIP ", new_hex.is_black);
            }
            if new_hex.is_black {
                new_map.insert(new_pos, new_hex);
            }
            // dbg!(new_map.len());
        }
    }

    new_map
}

fn main() {
    let data = read_data("input");
    let mut map = construct_map(&data.lines().collect());
    let count_one = count_black_tiles(&map);

    println!("Result #1: {}", count_one);

    for _ in 0..100 {
        map = iterate_day(&map);
    }

    let count_two = count_black_tiles(&map);
    println!("Result #2: {}", count_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_parse_line() {
        let str1 = "nwwswee";
        assert_eq!((0, 0), parse_line(&str1));

        let str2 = "esenee";
        assert_eq!((3, 0), parse_line(&str2));

        let str3 = "nenewwswswseseeene";
        assert_eq!((1, 1), parse_line(&str3));
    }

    #[test]
    fn test_count_black_tiles() {
        let map = construct_map(&TEST_DATA.lines().collect());
        let result = count_black_tiles(&map);

        assert_eq!(10, result);
    }

    #[test]
    fn test_count_black_neighbours() {
        let mut map = HexagonMap::new();
        let black_tile = Hexagon { is_black: true };
        let white_tile = Hexagon { is_black: false };
        map.insert((0, 0), black_tile);
        map.insert((-1, 0), black_tile);
        map.insert((-1, -1), white_tile);
        map.insert((1, -2), black_tile);
        count_black_neighbours(&map, &(0, -1));

        assert_eq!(3, count_black_neighbours(&map, &(0, -1)));

        assert_eq!(1, count_black_neighbours(&map, &(0, 0)));

        assert_eq!(1, count_black_neighbours(&map, &(-2, 0)));

        assert_eq!(0, count_black_neighbours(&map, &(-4, -4)));
    }

    #[test]
    fn test_iterate_day() {
        let mut map = construct_map(&TEST_DATA.lines().collect());
        assert_eq!(10, count_black_tiles(&map));

        map = iterate_day(&map);
        assert_eq!(15, count_black_tiles(&map));

        map = iterate_day(&map);
        assert_eq!(12, count_black_tiles(&map));

        map = iterate_day(&map);
        assert_eq!(25, count_black_tiles(&map));

        for _ in 0..97 {
            map = iterate_day(&map);
        }
        assert_eq!(2208, count_black_tiles(&map));
    }
}
