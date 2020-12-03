use std::fs::File;
use std::io;
use std::io::BufRead;

fn read_lines(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename);
    let reader = file.map(|e| io::BufReader::new(e).lines());
    let mut result = Vec::new();
    if let Ok(lines) = reader {
        for line in lines {
            if let Ok(l) = line {
                let line_vec = l.chars().collect();
                result.push(line_vec);
            }
        }
    }

    result
}

fn count_trees(data: &Vec<Vec<char>>, x_vel: usize, y_vel: usize) -> usize {
    let size_y = data.len();
    let size_x = data.get(0).unwrap().len();

    let (mut cur_x, mut cur_y) = (0, 0);
    let mut num_trees = 0;

    loop {
        cur_x = (cur_x + x_vel) % size_x;
        cur_y += y_vel;

        if cur_y >= size_y { break; }

        if data[cur_y][cur_x] == '#' {
            num_trees += 1;
        }
    }

    num_trees
}

fn main() {
    let data = read_lines("input");
    let num_trees_one = count_trees(&data, 3, 1);
    println!("Trees for part 1: {}", num_trees_one);

    let params: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let num_trees_two = params.into_iter()
        .map( |(x, y)| count_trees(&data, x, y) )
        .fold(1, |acc, e| acc * e);

    println!("Trees for part 2: {}", num_trees_two);
}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]

}
