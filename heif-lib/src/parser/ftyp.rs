use nom::{multi::many0, number::complete::be_u32, sequence::Tuple, IResult};

use crate::boxes::{generic::GenericBox, image::FileTypeBox};

use super::util::take_4b_str;

fn parse_brands(i: &[u8]) -> IResult<&[u8], Vec<&str>> {
    many0(take_4b_str)(i)
}

pub fn parse_file_type_box(base_box: GenericBox) -> IResult<&[u8], FileTypeBox> {
    let i = base_box.data;

    let (i, (major_brand, minor_version, compatible_brands)) =
        (take_4b_str, be_u32, parse_brands).parse(i)?;

    Ok((
        i,
        FileTypeBox {
            size: base_box.size,
            box_type: base_box.box_type,
            major_brand,
            minor_version,
            compatible_brands,
        },
    ))
}
