extern crate regex;
use core::panic;
use regex::Regex;

pub fn get_input() -> String{
    let mut input: String = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string().replace(" ", "")
}


pub fn form_postfix_notation_string(infix_notation: String,
                                        operators_priorities: std::collections::HashMap<char, i32>) -> String{
    check_infix_input_string(infix_notation.clone());

    let mut stack: Vec<char> = Vec::new();
    let mut postfix_notation: String = String::default();
    for c in infix_notation.chars() {
        if c.is_numeric() {
            postfix_notation.push(c);
        } else {
            if c == '('{
                stack.push(c);
                continue;
            }
            if c == ')' {
                while stack.last().cloned().unwrap() != '(' {
                    postfix_notation.push(' ');
                    postfix_notation.push(stack.last().cloned().unwrap());
                    stack.pop();
                }
                stack.pop();
                continue;
            }
            while stack.len() > 0 && stack.last().cloned().unwrap() != '(' && operators_priorities[&c] >= operators_priorities[stack.last().unwrap()]{
                postfix_notation.push(' ');
                postfix_notation.push(stack.last().cloned().unwrap());
                stack.pop();
            }
            postfix_notation.push(' ');
            stack.push(c);
        }
    }
    while !stack.is_empty() {
        postfix_notation.push(' ');
        postfix_notation.push(stack.last().cloned().unwrap());
        stack.pop();
    }
    postfix_notation
}

pub fn calculate_postfix(mut postfix_notation: String) -> String {
    let re = Regex::new(r"(?<f>[+-]?\d+) (?<s>[+-]?\d+) (?<o>[\^*/+-])").unwrap();

    loop {
        if let Some(caps) = re.captures(&postfix_notation) {
            let first = caps.name("f").unwrap().as_str().parse::<u32>().unwrap();
            let second = caps.name("s").unwrap().as_str().parse::<u32>().unwrap();
            let operator = caps.name("o").unwrap().as_str();
            let start_pos = caps.get(0).unwrap().start();
            let end_pos = caps.get(0).unwrap().end();

            let result = calculate_operation(first, second, operator);
            postfix_notation.replace_range(start_pos..end_pos, &result.to_string());
        } else {
            break;
        }
    }

    postfix_notation
}

fn calculate_operation(first: u32, second: u32, operator: &str) -> u32 {
    match operator {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" => first / second,
        "^" => first.pow(second),
        _ => panic!("Unknown operator"),
    }
}

fn check_braces(infix_notation: String) -> char{
    let mut counter: i32= 0;
    for c in infix_notation.chars() {
        if c == '(' {
            counter += 1;
        }
        if c == ')' {
            counter -= 1;
        }
        if counter < 0 {
            return '(';
        }
    }
    if counter > 0 {
        return ')';
    }
    return Default::default();
}

fn check_infix_input_string(infix_notation: String){
    let first_element: char = infix_notation.chars().next().unwrap();
    if !first_element.is_numeric() && first_element != '(' {
        panic!("Problem must start with operands!");
    }
    if infix_notation.chars().any(|c| matches!(c, 'a'..='z')) {
        panic!("No letters allowed!");
    }
    let braces_checker_result: char = check_braces(infix_notation.clone());
    if !(braces_checker_result == char::default()){
        panic!("'{}' is needed", braces_checker_result)
    }
}