use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
struct State {
    player1: Vec<usize>,
    player2: Vec<usize>,
}

fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn parse_data(data: &str) -> State {
    let mut state = State { player1: vec![], player2: vec![] };
    let mut splitted = data.split("\n\n");
    for line in splitted.next().unwrap().lines().skip(1) {
        state.player1.push(line.parse().unwrap());
    }

    for line in splitted.next().unwrap().lines().skip(1) {
        state.player2.push(line.parse().unwrap());
    }

    state
}

fn iterate(state: &mut State) -> bool {
    if state.player1.len() == 0 || state.player2.len() == 0 { return true; }
    let card1 = state.player1.remove(0);
    let card2 = state.player2.remove(0);

    if card1 > card2 {
        state.player1.push(card1);
        state.player1.push(card2);
    } else {
        state.player2.push(card2);
        state.player2.push(card1);
    }

    false
}

fn play_until_end(state: &mut State) -> usize {
    let mut finished = false;
    let mut num_rounds = 0;
    while !finished {
        finished = iterate(state);
        num_rounds += 1;
    }
    num_rounds
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Winner {
    NONE,
    P1,
    P2,
    RECURSE,
}


#[derive(PartialEq, Clone, Debug)]
struct RoundStatus {
    card1: usize,
    card2: usize,
    winner: Winner,
}

fn iterate_recursive_combat(state: &mut State) -> RoundStatus {
    let card1 = state.player1.remove(0);
    let card2 = state.player2.remove(0);
    let mut status = RoundStatus {
        card1,
        card2,
        winner: Winner::P1,
    };

    if state.player1.len() < card1 || state.player2.len() < card2 {
        if card1 < card2 {
            status.winner = Winner::P2;
        }
    } else {
        status.winner = Winner::RECURSE;
    }

    status
}

fn recursive_combat(starting_state: &mut State) -> Winner {
    let mut state = starting_state.clone();
    let mut prev_states: Vec<State> = Vec::new();
    let mut state_stack: VecDeque<(State, Vec<State>, RoundStatus)> = VecDeque::new();
    loop {
        let encountered = prev_states.iter()
            .any(|prev_state| {
                &state == prev_state
            });
        if encountered {
            process_win(&mut state, &mut prev_states, &mut state_stack, Winner::P1);
        };

        prev_states.push(state.clone());
        let round_status = iterate_recursive_combat(&mut state);

        match &round_status {
            &RoundStatus { card1, card2, winner } if winner == Winner::P1 => {
                state.player1.push(card1);
                state.player1.push(card2);
            }
            &RoundStatus { card1, card2, winner } if winner == Winner::P2 => {
                state.player2.push(card2);
                state.player2.push(card1);
            }
            &RoundStatus { card1, card2, winner } if winner == Winner::RECURSE => {
                state_stack.push_front((state.clone(), prev_states.clone(), round_status.clone()));
                state = State {
                    player1: state.player1.iter().take(card1).cloned().collect(),
                    player2: state.player2.iter().take(card2).cloned().collect(),
                };
                prev_states = Vec::new();
            }
            _ => panic!()
        }

        let winner = if state.player1.is_empty() { Winner::P2 } else if state.player2.is_empty() { Winner::P1 } else { Winner::NONE };

        if winner != Winner::NONE {
            if state_stack.is_empty() {
                *starting_state = state;
                return winner;
            } else {
                // We are in a sub-game
                process_win(&mut state, &mut prev_states, &mut state_stack, winner)
            }
        }
    }
}

fn process_win(state: &mut State, prev_states: &mut Vec<State>, state_stack: &mut VecDeque<(State, Vec<State>, RoundStatus)>, winner: Winner) {
    let prev = state_stack.pop_front().unwrap();
    *state = prev.0;
    *prev_states = prev.1;
    let round_status = prev.2;

    match winner {
        Winner::P1 => {
            state.player1.push(round_status.card1);
            state.player1.push(round_status.card2);
        }
        Winner::P2 => {
            state.player2.push(round_status.card2);
            state.player2.push(round_status.card1);
        }
        _ => panic!()
    }
}

fn compute_score(state: &State) -> Option<usize> {
    if state.player1.len() != 0 && state.player2.len() != 0 { return None; }
    let winning_hand = if state.player1.len() == 0 { &state.player2 } else { &state.player1 };

    Some(winning_hand.iter()
        .rev()
        .enumerate()
        .map(|(idx, card)| (idx + 1) * card)
        .sum())
}

fn main() {
    let data = read_data("input");
    let initial_state = parse_data(&data);
    let mut state_one = initial_state.clone();
    let num_rounds = play_until_end(&mut state_one);
    println!("Number of rounds played: {}", num_rounds);

    let score_one = compute_score(&state_one).unwrap();
    println!("Result #1: {}", score_one);

    let mut state_two = initial_state.clone();
    recursive_combat(&mut state_two);
    let score_two = compute_score(&state_two).unwrap();
    println!("Result #2: {}", score_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA1: &'static str = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_parse_data() {
        let state = parse_data(&TEST_DATA1);

        assert_eq!(
            vec![9, 2, 6, 3, 1],
            state.player1
        );

        assert_eq!(
            vec![5, 8, 4, 7, 10],
            state.player2
        );
    }

    #[test]
    fn test_iterate() {
        let mut state = parse_data(&TEST_DATA1);

        iterate(&mut state);

        assert_eq!(
            vec![2, 6, 3, 1, 9, 5],
            state.player1
        );

        assert_eq!(
            vec![8, 4, 7, 10],
            state.player2
        );

        iterate(&mut state);

        assert_eq!(
            vec![6, 3, 1, 9, 5],
            state.player1
        );

        assert_eq!(
            vec![4, 7, 10, 8, 2],
            state.player2
        );
    }

    #[test]
    fn test_play_until_end() {
        let mut state = parse_data(&TEST_DATA1);

        let num_rounds = play_until_end(&mut state);

        assert_eq!(
            30,
            num_rounds
        );

        assert_eq!(
            0,
            state.player1.len()
        );

        assert_eq!(
            10,
            state.player2.len()
        );

        assert_eq!(
            vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1],
            state.player2
        );

        let score = compute_score(&state);

        assert_eq!(
            Some(306),
            score
        );
    }

    #[test]
    fn test_iterate_recursive_combat() {
        let mut state = parse_data(&TEST_DATA1);

        let mut round_status = iterate_recursive_combat(&mut state);

        assert_eq!(
            RoundStatus {
                card1: 9,
                card2: 5,
                winner: Winner::P1,
            },
            round_status
        );

        assert_eq!(
            vec![2, 6, 3, 1],
            state.player1
        );

        assert_eq!(
            vec![8, 4, 7, 10],
            state.player2
        );

        state.player1 = vec![4, 3, 8, 5, 2];
        state.player2 = vec![3, 10, 1, 7, 6];

        round_status = iterate_recursive_combat(&mut state);

        assert_eq!(
            RoundStatus {
                card1: 4,
                card2: 3,
                winner: Winner::RECURSE,
            },
            round_status
        );
    }

    #[test]
    fn test_recursive_combat() {
        let mut state = parse_data(&TEST_DATA1);

        let winner = recursive_combat(&mut state);

        assert_eq!(
            Vec::<usize>::new(),
            state.player1
        );

        assert_eq!(
            vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3],
            state.player2
        );

        assert_eq!(
            Winner::P2,
            winner
        );

        let score = compute_score(&state).unwrap();

        assert_eq!(
            291,
            score
        );
    }
}