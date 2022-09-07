use std::fs::File;
use std::io::Error;
use std::io::Write;

pub fn output(file_name: String, content: String) -> Result<(), Option<Error>> {
    let mut file = File::create(file_name).expect("something error");
    file.write_all(content.as_bytes()).expect("something error");

    Ok(())
}
