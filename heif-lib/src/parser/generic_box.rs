use nom::{
    bytes::complete::take,
    error::dbg_dmp,
    number::complete::{be_u32, be_u8},
    sequence::Tuple,
    IResult,
};

use super::util::take_4b_str;
use crate::boxes::generic::{GenericBox, GenericFullBox};

fn take_size(i: &[u8]) -> IResult<&[u8], u32> {
    be_u32(i)
}

fn take_data(box_size: u32) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    // Remove 8 bytes to account for the box_size and box_type values that were already read
    move |i: &[u8]| take((box_size - 8) as usize)(i)
}

pub fn parse_base_box(i: &[u8]) -> IResult<&[u8], GenericBox> {
    // TODO, spcial size case
    // https://github.com/jdeng/goheif/blob/a0d6a8b3e68f9d613abd9ae1db63c72ba33abd14/heif/bmff/bmff.go#L199

    let (i, (box_size, box_type)) = (take_size, dbg_dmp(take_4b_str, "bruh")).parse(i)?;
    let (i, data) = take_data(box_size)(i)?;

    Ok((
        i,
        GenericBox {
            size: box_size,
            box_type,
            data,
        },
    ))
}

pub fn parse_boxes(i: &[u8]) -> IResult<&[u8], Vec<GenericBox>> {
    let mut input_left = i;
    let mut boxes = Vec::new();

    while input_left.len() > 8 {
        let (new_i, parsed_box) = parse_base_box(input_left)?;

        input_left = new_i;
        boxes.push(parsed_box)
    }

    Ok((input_left, boxes))
}

fn take_version(i: &[u8]) -> IResult<&[u8], u8> {
    be_u8(i)
}

fn take_flags(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, b) = take(3_usize)(i)?;
    let padded = [0, b[0], b[1], b[2]];
    let n = u32::from_be_bytes(padded);

    Ok((i, n))
}

/// IMPORTANT this does not parse the full box from binary
/// instead it continues parsing the data of a base box
pub fn parse_full_box(base_box: GenericBox) -> IResult<&[u8], GenericFullBox> {
    // TODO, spcial size case
    // https://github.com/jdeng/goheif/blob/a0d6a8b3e68f9d613abd9ae1db63c72ba33abd14/heif/bmff/bmff.go#L199
    let i = base_box.data;

    let (i, (version, flags)) = (take_version, take_flags).parse(i)?;

    // let (i, data) = take_while(|_| true).parse(i)?;

    Ok((
        i,
        GenericFullBox {
            size: base_box.size,
            box_type: base_box.box_type,
            version,
            flags,
            // The remaining data is just i
            data: i,
        },
    ))
}
