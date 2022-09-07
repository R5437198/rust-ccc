mod file_utils;

fn main() {
    let file_name = "pizza.s".to_string();
    let content = "hello, pizza".to_string();

    match file_utils::output(file_name, content) {
        Ok(_) => println!("Success."),
        Err(_) => println!("Error."), 
    }
}