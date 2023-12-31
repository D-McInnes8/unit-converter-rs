use std::io;

fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    return;
                } else {
                    let command = remove_new_line_characters(&input);
                    if command == "exit" {
                        return;
                    }
                    println!("{}", command);
                }
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }
}

fn remove_new_line_characters(input: &String) -> &str {
    let result = input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input);
    return result;
}
