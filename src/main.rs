mod functions;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    print!(">>>");
    io::stdout().flush().unwrap();

    let operators: HashMap<char, i32> = HashMap::from([
        ('^', 1i32),
        ('*', 2i32),
        ('/', 2i32),
        ('+', 3i32),
        ('-', 3i32)
    ]);

    let postfix_notation: String = functions::form_postfix_notation_string(functions::get_input(), operators);

    println!("{}", functions::calculate_postfix(postfix_notation));
}
