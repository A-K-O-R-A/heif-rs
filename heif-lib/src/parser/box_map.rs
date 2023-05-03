use nom::IResult;

use crate::boxes::{generic::GenericBox, parsed::ParsedBox};

use super::boxes::{ftyp::parse_file_type_box, meta::parse_meta_box};

pub fn parse_generic_box(base_box: GenericBox) -> IResult<&[u8], ParsedBox> {
    // TODO, spcial size case
    // https://github.com/jdeng/goheif/blob/a0d6a8b3e68f9d613abd9ae1db63c72ba33abd14/heif/bmff/bmff.go#L199
    let i = base_box.data;
    let box_type = base_box.box_type;

    let parsed_box: ParsedBox = match box_type {
        "meta" => ParsedBox::MetaBox(parse_meta_box(base_box)?.1),
        "ftyp" => ParsedBox::FileTypeBox(parse_file_type_box(base_box)?.1),
        _ => ParsedBox::Box(base_box),
    };

    Ok((i, parsed_box))
}
