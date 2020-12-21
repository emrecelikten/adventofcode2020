// This one was not super great.

fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn parse_data(data: &str) -> Vec<String> {
    data.lines().map(|x| x.to_owned()).collect()
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Token {
    Number(i64),
    Add,
    Mul,
    LParen,
    RParen,
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut token_str = line.replace("(", "( ");
    token_str = token_str.replace(")", " )");
    let tokens = token_str.split(" ").filter(|x| !x.is_empty());
    tokens
        .map(|s| match s {
            "+" => Token::Add,
            "*" => Token::Mul,
            "(" => Token::LParen,
            ")" => Token::RParen,
            _ => Token::Number(s.parse().unwrap()),
        })
        .collect()
}

fn find_parentheses_pos(tokens: &[Token]) -> Option<(usize, usize)> {
    let mut start = 0;
    let mut end = 0;
    for (i, ch) in tokens.iter().enumerate() {
        if *ch == Token::LParen {
            start = i;
        } else if *ch == Token::RParen {
            end = i;
            break;
        }
    }
    if end != 0 {
        Some((start, end))
    } else {
        None
    }
}

fn eval_one(tokens: &[Token]) -> i64 {
    let mut iter = tokens.iter();
    let mut result = 0;
    if let Token::Number(num) = iter.nth(0).unwrap() {
        result = *num;
        while let Some(op_token) = iter.next() {
            if let Some(Token::Number(next_num)) = iter.next() {
                match op_token {
                    Token::Add => result += next_num,
                    Token::Mul => result *= next_num,
                    _ => {
                        panic!("Malformed input: {:?}", &tokens);
                    }
                }
            } else {
                panic!("Malformed input: {:?}", &tokens);
            }
        }
    }

    result
}

fn eval_two(tokens: &[Token]) -> i64 {
    let mut cur_tokens = tokens.to_vec();

    while cur_tokens.contains(&Token::Add) {
        let mut new_tokens = Vec::new();
        for i in 0..cur_tokens.len() {
            if cur_tokens[i] == Token::Add {
                if let (Token::Number(n1), Token::Number(n2)) =
                    (&cur_tokens[i - 1], &cur_tokens[i + 1])
                {
                    new_tokens.push(Token::Number(n1 + n2));
                    if i + 2 < cur_tokens.len() {
                        new_tokens.extend_from_slice(&cur_tokens[i + 2..]);
                    }
                    break;
                }
            } else if cur_tokens[i] == Token::Mul {
                new_tokens.extend_from_slice(&cur_tokens[i - 1..=i]);
            }
        }
        cur_tokens = new_tokens;
    }

    eval_one(&cur_tokens)
}

fn process(tokens: &[Token], function: &dyn Fn(&[Token]) -> i64) -> i64 {
    let mut cur_tokens: Vec<Token> = tokens.to_vec();
    let mut new_tokens: Vec<Token> = Vec::new();
    while let Some((start, end)) = find_parentheses_pos(&cur_tokens) {
        // dbg!(&cur_line[start..=end]);
        let result = function(&cur_tokens[start + 1..end]);
        new_tokens.extend_from_slice(&cur_tokens[..start]);
        new_tokens.push(Token::Number(result));
        new_tokens.extend_from_slice(&cur_tokens[end + 1..]);
        cur_tokens = new_tokens;
        new_tokens = Vec::new();
    }
    function(&cur_tokens)
}

fn main() {
    let lines: Vec<Vec<Token>> = parse_data(&read_data("input"))
        .iter()
        .map(|line| tokenize(&line))
        .collect();
    let result_one: i64 = lines.iter().map(|line| process(&line, &eval_one)).sum();
    println!("Result #1: {}", result_one);

    let result_two: i64 = lines.iter().map(|line| process(&line, &eval_two)).sum();
    println!("Result #2: {}", result_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &'static str = r"1 + (2 * 3) + (4 * (5 + 6))";
    const TEST_DATA_2: &'static str = r"2 * 3 + (4 * 5)";
    const TEST_DATA_3_PARTIAL: &'static str = r"8 * 3 + 9 + 3 * 4 * 3";
    const TEST_DATA_4: &'static str = r"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const TEST_DATA_5: &'static str = r"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_tokenize() {
        let result = tokenize(&TEST_DATA_1);
        assert_eq!(result[0], Token::Number(2));
        assert_eq!(result[1], Token::Mul);
        assert_eq!(result[2], Token::Number(3));
        assert_eq!(result[3], Token::Add);
        assert_eq!(result[4], Token::LParen);
        assert_eq!(result[5], Token::Number(4));
        assert_eq!(result[6], Token::Mul);
        assert_eq!(result[7], Token::Number(5));
        assert_eq!(result[8], Token::RParen);
        assert_eq!(result.len(), 9);
    }

    #[test]
    fn test_find_parentheses_pos() {
        let (start, end) = find_parentheses_pos(&tokenize(&TEST_DATA_5)).unwrap();
        assert_eq!(start, 1);
        assert_eq!(end, 7);
    }

    #[test]
    fn test_eval_one() {
        let result = eval_one(&tokenize(&TEST_DATA_3_PARTIAL));
        assert_eq!(result, 432);
    }

    #[test]
    fn test_eval_two() {
        let result = eval_two(&tokenize(&TEST_DATA_3_PARTIAL));
        assert_eq!(result, 1440);
    }

    #[test]
    fn test_process() {
        let result_2_partial = process(&tokenize(&TEST_DATA_3_PARTIAL), &eval_one);
        assert_eq!(result_2_partial, 432);

        let result_1_2 = process(&tokenize(&TEST_DATA_1), &eval_two);
        assert_eq!(result_1_2, 51);

        let result_2_2 = process(&tokenize(&TEST_DATA_2), &eval_two);
        assert_eq!(result_2_2, 46);

        let result_4_2 = process(&tokenize(&TEST_DATA_4), &eval_two);
        assert_eq!(result_4_2, 669060);

        let result_5 = process(&tokenize(&TEST_DATA_5), &eval_one);
        assert_eq!(result_5, 13632);

        let result_5_2 = process(&tokenize(&TEST_DATA_5), &eval_two);
        assert_eq!(result_5_2, 23340);
    }
}
