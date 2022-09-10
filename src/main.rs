mod output;
use std::io;

fn main() {
    let file_name = "pizza.s".to_string();
    let content = receive_input();

    match output::output(file_name, content) {
        Ok(_) => println!("Success."),
        Err(_) => println!("Error."),
    }
}

fn receive_input() -> String {
    println!("Type Number >>");
    
    let default_value = 15.to_string();
    let mut input = String::new();
    let result: std::io::Result<usize> = io::stdin().read_line(&mut input);
    input = input.trim().to_string();

    match result {
        Ok(_) => {
            return validate_input(input);
        },
        Err(e) => {
            println!("Use default Value. {}", e);
            return default_value;
        },
    }
}

fn validate_input(input: String) -> String {
    let default_value = 15.to_string();

    match input.trim().parse::<i32>() {
        Ok(_) => input,
        Err(e) => {
            println!("Use default Value, {}", e);
            return default_value;
        },
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_input_test(){
        assert_eq!(validate_input(18.to_string()), "18".to_string());
        assert_eq!(validate_input("a".to_string()), "15".to_string());
    }
}