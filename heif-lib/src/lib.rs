use std::io::{self, Read};

pub mod parser;
pub mod types;

pub fn parse_file<F: Read>(file: &mut F) -> io::Result<()> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    match parser::parse_box(&buf) {
        Ok((_i, first_box)) => println!("First box: {}", first_box),
        Err(e) => panic!("{e}"),
    };

    Ok(())
}
