use std::io::{self, Read};

use boxes::{generic::GenericBox, parsed::ParsedBox};
use parser::boxes::{ftyp::parse_file_type_box, meta::parse_meta_box};

use crate::parser::box_map::parse_generic_box;

pub mod boxes;
pub mod parser;

pub fn parse_file<F: Read>(file: &mut F) -> io::Result<()> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let parse_result = parser::generic_box::parse_boxes(&buf);

    if let Ok((_i, boxes)) = parse_result {
        //println!("{}", boxe);
        //println!("{:x?}", boxe.data);
        let parsed_boxes: Vec<_> = parse_generic_boxes(&boxes);

        let (_, _ftyp_box) = parse_file_type_box(boxes[0].clone()).unwrap();
        let (_, meta_box) = parse_meta_box(boxes[1].clone()).unwrap();

        let parsed_meta_childs = parse_generic_boxes(&meta_box.boxes);

        for b in parsed_boxes {
            println!("{:?}\n", b);
        }

        println!("\nMeta childs:");

        for b in parsed_meta_childs {
            println!("{:?}\n", b);
        }
    } else {
        panic!("e");
    }

    Ok(())
}

fn parse_generic_boxes<'a>(boxes: &[GenericBox<'a>]) -> Vec<ParsedBox<'a>> {
    boxes
        .iter()
        .map(|b| parse_generic_box(*b).unwrap().1)
        .collect()
}
