fn parse_line(line: &str) -> (usize, usize) {
    let row: String = line.chars()
        .take_while(|&e| e == 'F' || e == 'B')
        .map(|e| if e == 'F' { '0' } else { '1' }).collect();
    let row_int = usize::from_str_radix(&row, 2).unwrap();

    let col: String = line.chars()
        .skip_while(|&e| e == 'F' || e == 'B')
        .map(|e| if e == 'L' { '0' } else { '1' }).collect();
    let col_int = usize::from_str_radix(&col, 2).unwrap();

    (row_int, col_int)
}

fn sum_interval(lower: usize, upper: usize) -> usize {
    (upper * (upper + 1) / 2) - (lower * (lower + 1) / 2)
}

pub fn method_two(lines: impl Iterator<Item=String>) {
    let (mut min, mut max) = (usize::max_value(), usize::min_value());
    let mut sum = 0;

    for line in lines {
        let (row, col) = parse_line(&line);
        let id = row * 8 + col;

        if min > id {
            min = id
        }

        if max < id {
            max = id
        }

        sum += id;
    }

    println!("Max: {}", max);
    println!("Missing: {}", sum_interval(min, max) - sum)
}