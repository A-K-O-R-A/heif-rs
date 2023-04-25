use std::io::{self, Read};

use parser::meta_box::parse_meta_box;

pub mod boxes;
pub mod parser;

pub fn parse_file<F: Read>(file: &mut F) -> io::Result<()> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let parse_result = parser::base_box::parse_boxes(&buf);

    if let Ok((_i, boxes)) = parse_result {
        //println!("{}", boxe);
        //println!("{:x?}", boxe.data);
        let (_, meta_box) = parse_meta_box(boxes[1].clone()).unwrap();

        for b in meta_box.boxes {
            println!("{}", b);
        }

        for b in boxes {
            println!("{}", b);
        }
    } else {
        panic!("e");
    }

    Ok(())
}
