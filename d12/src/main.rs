struct Command {
    op: char,
    arg: i32,
}

#[derive(Debug, PartialEq)]
struct ShipState {
    xpos: i32,
    ypos: i32,
    heading: i32,
    wp_xpos: i32,
    wp_ypos: i32,
}

fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn parse_data(data: &str) -> Vec<Command> {
    data.lines().map(|line| {
        let op = line.chars().nth(0).unwrap();
        let arg: i32 = line[1..].parse().unwrap();
        Command { op: op, arg: arg }
    }).collect()
}

fn manhattan(state: &ShipState) -> i32 {
    state.xpos.abs() + state.ypos.abs()
}

fn eval_commands_one(commands: &[Command]) -> ShipState {
    let mut state = ShipState { xpos: 0, ypos: 0, heading: 0, wp_xpos: 10, wp_ypos: 1 };
    for command in commands {
        match command.op {
            'N' => state.ypos += command.arg,
            'S' => state.ypos -= command.arg,
            'E' => state.xpos += command.arg,
            'W' => state.xpos -= command.arg,
            'L' => state.heading = (state.heading - command.arg + 360) % 360, // This won't work for large neg. arg
            'R' => state.heading = (state.heading + command.arg + 360) % 360,
            'F' => match state.heading {
                0 => state.xpos += command.arg,
                90 => state.ypos -= command.arg,
                180 => state.xpos -= command.arg,
                270 => state.ypos += command.arg,
                x => panic!("Unexpected heading: {}!", x),
            },
            x => panic!("Unexpected op: {}!", x),
        }
    }

    state
}

fn rotate(state: &mut ShipState, command: &Command) {
    assert!(command.op == 'L' || command.op == 'R');
    let direction = if command.op == 'R' { -1 } else { 1 };
    let euclidean = ((state.wp_xpos.pow(2) + state.wp_ypos.pow(2)) as f64).sqrt();
    let cur_angle = (state.wp_ypos as f64).atan2(state.wp_xpos as f64);
    let new_angle = cur_angle + ((command.arg * direction) as f64 * std::f64::consts::PI) / 180.0;

    state.wp_ypos = (new_angle.sin() * euclidean).round() as i32;
    state.wp_xpos = (new_angle.cos() * euclidean).round() as i32;
}

fn eval_commands_two(commands: &[Command]) -> ShipState {
    let mut state = ShipState { xpos: 0, ypos: 0, heading: 0, wp_xpos: 10, wp_ypos: 1 };
    for command in commands {
        match command.op {
            'N' => state.wp_ypos += command.arg,
            'S' => state.wp_ypos -= command.arg,
            'E' => state.wp_xpos += command.arg,
            'W' => state.wp_xpos -= command.arg,
            'L' => rotate(&mut state, &command),
            'R' => rotate(&mut state, &command),
            'F' => {
                state.ypos += command.arg * state.wp_ypos;
                state.xpos += command.arg * state.wp_xpos;
            }
            _ => panic!("Unexpected op!")
        }
    }

    state
}

fn main() {
    let data = parse_data(&read_data("input"));
    let ship_state1 = eval_commands_one(&data);
    let distance1 = manhattan(&ship_state1);
    println!("Distance from origin #1: {}", distance1);

    let ship_state2 = eval_commands_two(&data);
    let distance2 = manhattan(&ship_state2);
    println!("Distance from origin #2: {}", distance2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = r"F10
N3
F7
R90
F11";

    #[test]
    fn test_eval_commands_one() {
        let data = parse_data(&TEST_DATA);
        let result = eval_commands_one(&data);
        assert_eq!(result, ShipState {
            xpos: 17,
            ypos: -8,
            heading: 90,
            wp_xpos: 10,
            wp_ypos: 1,
        });
    }

    #[test]
    fn test_rotate() {
        let mut ship_state = ShipState {
            xpos: 0,
            ypos: 0,
            heading: 0,
            wp_xpos: 10,
            wp_ypos: 1,
        };

        let command1 = Command { op: 'R', arg: 90 };
        let command2 = Command { op: 'R', arg: -180 };
        let command3 = Command { op: 'L', arg: 90 };

        rotate(&mut ship_state, &command1);
        assert_eq!(ship_state.wp_xpos, 1);
        assert_eq!(ship_state.wp_ypos, -10);

        rotate(&mut ship_state, &command2);
        assert_eq!(ship_state.wp_xpos, -1);
        assert_eq!(ship_state.wp_ypos, 10);

        rotate(&mut ship_state, &command3);
        assert_eq!(ship_state.wp_xpos, -10);
        assert_eq!(ship_state.wp_ypos, -1);
    }

    #[test]
    fn test_eval_commands_two() {
        let data = parse_data(&TEST_DATA);
        let result = eval_commands_two(&data);

        assert_eq!(result, ShipState {
            xpos: 214,
            ypos: -72,
            heading: 0,
            wp_xpos: 4,
            wp_ypos: -10,
        });
    }
}