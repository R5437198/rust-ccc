use std::fs::File;
use std::io::Error;
use std::io::Write;

pub fn output(file_name: String, content: String) -> Result<(), Option<Error>> {
    let mut file = File::create(file_name).expect("something error");
    let mut _content = get_header() + &get_wrapper_header() + &get_content(content) + &get_footer();

    file.write_all(_content.as_bytes())
        .expect("something error");

    Ok(())
}

fn get_header() -> String {
    let header = ".intel_syntax noprefix\n.globl main\n\n".to_string();
    header
}

fn get_wrapper_header() -> String {
    let wrapper_header = "main:\n".to_string();
    wrapper_header
}

fn get_content(input: String) -> String {
    let content = format!("     mov rax, {input}\n").to_string();
    content
}

fn get_footer() -> String {
    let footer = "     ret".to_string();
    footer
}
