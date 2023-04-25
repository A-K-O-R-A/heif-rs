use nom::{bytes::complete::take, number::complete::be_u32, sequence::Tuple, IResult, Parser};

use crate::boxes::base::BaseBox;

fn take_size(i: &[u8]) -> IResult<&[u8], u32> {
    be_u32(i)
}

fn take_type(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take(4usize)(i)
}

fn take_data(box_size: u32) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    // Remove 8 bytes to account for the box_size and box_type values that were already read
    move |i: &[u8]| take((box_size - 8) as usize)(i)
}

pub fn parse_box(i: &[u8]) -> IResult<&[u8], BaseBox> {
    let (i, (box_size, box_type)) = (take_size, take_type).parse(i)?;
    let (i, data) = take_data(box_size).parse(i)?;

    Ok((
        i,
        BaseBox {
            size: box_size,
            box_type,
            data,
        },
    ))
}

pub fn parse_boxes(i: &[u8]) -> IResult<&[u8], Vec<BaseBox>> {
    let mut input_left = i;
    let mut boxes = Vec::new();

    while input_left.len() > 8 {
        let (new_i, parsed_box) = parse_box(input_left)?;

        input_left = new_i;
        boxes.push(parsed_box)
    }

    Ok((input_left, boxes))
}
