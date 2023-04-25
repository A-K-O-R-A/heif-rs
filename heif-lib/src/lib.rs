use std::io::{self, Read};

pub mod parser;
pub mod boxes;

pub fn parse_file<F: Read>(file: &mut F) -> io::Result<()> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let parse_result = parser::parse_boxes(&buf);

    if let Ok((_i, boxes)) = parse_result {
        //println!("{}", boxe);
        //println!("{:x?}", boxe.data);

        for b in boxes {
            println!("{}", b);
        }
    } else {
        panic!("e");
    }

    Ok(())
}
