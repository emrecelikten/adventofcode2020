fn read_data(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|e| e.to_owned())
        .collect()
}

fn parse_data<I>(data: I) -> Vec<i64>
    where I: IntoIterator, I::Item: AsRef<str>
{
    data.into_iter().map(|e| e.as_ref().parse().unwrap()).collect()
}

fn part_one(data: &[i64]) -> usize {
    let mut sorted = data.to_owned();
    sorted.push(0);
    sorted.sort();
    sorted.push(sorted.last().unwrap() + 3);

    let diff: Vec<i64> = sorted.windows(2).map(|e| e[1] - e[0]).collect();
    let ones = diff.iter().filter(|&&e| e == 1).count();
    let threes = diff.iter().filter(|&&e| e == 3).count();
    ones * threes
}

fn part_two(data: &[i64]) -> usize {
    let mut sorted = data.to_owned();
    sorted.push(0);
    sorted.sort();
    sorted.push(sorted.last().unwrap() + 3);

    let mut num_arrangements = vec![0usize; sorted.len()];
    let len = sorted.len();
    num_arrangements[len - 1] = 1;

    for idx in (0..len - 1).rev() {
        let mut count = 0;
        for i in idx + 1..(idx + 4).min(len) {
            if sorted[i] - sorted[idx] <= 3 {
                count += num_arrangements[i];
            }
        }
        num_arrangements[idx] = count;
    }

    num_arrangements[0]
}

fn main() {
    let str = read_data("input");
    let data = parse_data(&str);
    let part_one_result = part_one(&data);
    println!("Multiplication result: {}", part_one_result);

    let part_two_result = part_two(&data);
    println!("Distinct ways result: {}", part_two_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA2: &'static str = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part_one() {
        let data = parse_data(TEST_DATA2.lines());
        assert_eq!(data[0], 28);
        assert_eq!(data[data.len() - 1], 3);

        let result = part_one(&data);
        assert_eq!(result, 220);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data(TEST_DATA2.lines());

        let result = part_two(&data);
        assert_eq!(result, 19208);
    }
}
