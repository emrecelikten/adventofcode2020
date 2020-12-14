fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn parse_data(data: &str) -> (i32, Vec<i32>) {
    let mut lines = data.lines();
    let cur_time = lines.nth(0).unwrap().parse().unwrap();
    let bus_times = lines
        .nth(0)
        .unwrap()
        .split(",")
        .map(|x| {
            if x == "x" { -1 } else { x.parse().unwrap() }
        }).collect();

    (cur_time, bus_times)
}

fn find_earliest_bus(cur_time: &i32, bus_times: &[i32]) -> (i32, i32) {
    let (bus_id, min_wait) = bus_times
        .iter()
        .zip(bus_times.iter())
        .filter_map(|(i, &x)| if x != -1 { Some((i, x - cur_time % x)) } else { None })
        .min_by_key(|x| x.1)
        .unwrap();
    (*bus_id, min_wait)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut ia = a;
    let mut ib = b;
    while ib != 0 {
        let tmp = ia;
        ia = ib;
        ib = tmp % ib;
    }
    ia
}

fn find_earliest_time(bus_times: &[i32]) -> usize {
    let mut sorted_times: Vec<(usize, usize)> = bus_times.iter()
        .enumerate()
        .filter_map(|(idx, &x)| if x != -1 { Some((idx, x as usize)) } else { None })
        .collect();
    sorted_times.sort_by_key(|x| std::cmp::Reverse(x.1));

    let (largest_idx, largest_bus_id) = sorted_times.iter().nth(0).unwrap();
    let mut n = (largest_bus_id - largest_idx % largest_bus_id) % largest_bus_id;
    let mut step = *largest_bus_id;

    sorted_times.remove(0);

    loop {
        n += step;
        if let Some((idx, bus_id)) = sorted_times.first() {
            let mut time_until = bus_id - n % bus_id;
            if time_until == *bus_id { time_until -= bus_id; }

            if time_until != *idx % bus_id {
                continue;
            }

            step = (step * bus_id) / gcd(step, *bus_id);
            sorted_times.remove(0);
        } else { break; }
    }

    return n - step;
}

fn main() {
    let (cur_time, bus_times) = parse_data(&read_data("input"));

    let result_one = find_earliest_bus(&cur_time, &bus_times);
    println!("Result #1: {}", result_one.0 * result_one.1);

    let now = std::time::Instant::now();
    let result_two = find_earliest_time(&bus_times);
    let elapsed = now.elapsed();
    println!("Result #2: {}", result_two);
    dbg!(elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = r"939
7,13,x,x,59,x,31,19";

    const TEST_DATA_2: &'static str = r"0
1789,37,47,1889";

    #[test]
    fn test_find_earliest_bus() {
        let (cur_time, bus_times) = parse_data(&TEST_DATA);
        let (id, min_wait) = find_earliest_bus(&cur_time, &bus_times);
        assert_eq!(id, 59);
        assert_eq!(min_wait, 5);
    }

    #[test]
    fn test_find_earliest_time() {
        let (_, bus_times) = parse_data(&TEST_DATA);
        let result = find_earliest_time(&bus_times);
        assert_eq!(result, 1068781);
    }

    #[test]
    fn test_find_earliest_time_2() {
        let (_, bus_times) = parse_data(&TEST_DATA_2);
        let result = find_earliest_time(&bus_times);
        assert_eq!(result, 1202161486);
    }
}