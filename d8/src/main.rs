use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct Instruction<'a> {
    operation: &'a str,
    argument: i64,
}

struct State {
    pc: i64,
    acc: i64,
    ok: bool,
}

fn load_data(filename: &str) -> Vec<String> {
    let str = std::fs::read_to_string(filename).unwrap();
    str.split("\n").filter_map(|e| if !e.is_empty() { Some(e.to_owned()) } else { None }).collect()
}

fn parse_data<T: AsRef<str>>(data: &[T]) -> Vec<Instruction> {
    data.iter().map(|e| {
        let mut splitted = e.as_ref().split(" ");
        Instruction { operation: splitted.next().unwrap(), argument: i64::from_str(splitted.next().unwrap()).unwrap() }
    }).collect()
}

fn execute<'a>(instructions: &'a Vec<Instruction<'a>>) -> State {
    let mut indexes: Vec<bool> = vec![false; instructions.len()];
    let mut state = State { pc: 0, acc: 0, ok: true };
    loop {
        if state.pc as usize >= indexes.len() { return state; }
        if indexes[state.pc as usize] == true {
            state.ok = false;
            break;
        }
        indexes[state.pc as usize] = true;
        let inst = &instructions[state.pc as usize];
        state.pc += 1;
        match inst.operation {
            "nop" => { continue; }
            "acc" => { state.acc += inst.argument; }
            "jmp" => { state.pc += -1 + inst.argument; }
            other => { panic!("Unknown operation: {}", other); }
        }
    }
    state
}

fn fix<'a>(instructions: &'a Vec<Instruction<'a>>) -> Vec<Instruction<'a>> {
    for (i, inst) in instructions.iter().enumerate() {
        // No check for valid case for accs, since we know the input program is broken.
        if inst.operation == "acc" { continue; }
        let mut new_instructions: Vec<Instruction<'a>> = instructions.to_vec();

        if inst.operation == "nop" {
            new_instructions[i].operation = "jmp";
        } else {
            new_instructions[i].operation = "nop";
        }

        let result = execute(&new_instructions);
        if result.ok == true { return new_instructions; }
    }
    panic!("Unfixable program detected!");
}

fn main() {
    let data = load_data("input");
    let instructions = parse_data(&data);
    let part_one_state = execute(&instructions);
    println!("ACC value for part one: {}", part_one_state.acc);

    let fixed_instructions = fix(&instructions);
    let part_two_state = execute(&fixed_instructions);
    println!("ACC value for part two: {}", part_two_state.acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROGRAM: &'static str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_parse() {
        let data: Vec<&str> = PROGRAM.split("\n").collect();
        let instructions = parse_data(&data);
        assert_eq!(instructions[0], Instruction { operation: "nop", argument: 0 });
        assert_eq!(instructions[1], Instruction { operation: "acc", argument: 1 });
        assert_eq!(instructions[2], Instruction { operation: "jmp", argument: 4 });
        assert_eq!(instructions[4], Instruction { operation: "jmp", argument: -3 });
    }

    #[test]
    fn test_execute() {
        let data: Vec<&str> = PROGRAM.split("\n").collect();
        let instructions = parse_data(&data);
        let result = execute(&instructions);
        assert_eq!(result.acc, 5);
        assert_eq!(result.pc, 1);
        assert_eq!(result.ok, false);
    }

    #[test]
    fn test_fix() {
        let data: Vec<&str> = PROGRAM.split("\n").collect();
        let instructions = parse_data(&data);
        let fixed = fix(&instructions);
        let result = execute(&fixed);

        assert_eq!(result.acc, 8);
        assert_eq!(result.pc, 9);
        assert_eq!(result.ok, true);
    }
}