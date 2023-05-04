use nom::bytes::complete::take;
use nom::combinator::eof;
use nom::number::complete::{be_u16, be_u32};
use nom::sequence::Tuple;
use nom::IResult;

use crate::boxes::generic::GenericBox;
use crate::boxes::meta::{HandlerBox, MetaBox, PrimaryItemBox};

use crate::parser::generic_box::{parse_boxes, parse_full_box};
use crate::parser::util::take_all_str;

/// Parses a `"meta"` box from a given generic Box
pub fn parse_meta_box(base_box: GenericBox) -> IResult<&[u8], MetaBox> {
    let (_, full_box) = parse_full_box(base_box)?;

    let (i, child_boxes) = parse_boxes(full_box.data)?;

    Ok((
        i,
        MetaBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            boxes: child_boxes,
        },
    ))
}

/// Parses a `"hdlr"` box from a given generic Box
pub fn parse_handler_box(base_box: GenericBox) -> IResult<&[u8], HandlerBox> {
    let (_, full_box) = parse_full_box(base_box)?;
    let i = full_box.data;

    let (i, (handler_type, name, _)) = (take(4usize), take_all_str, eof).parse(i)?;

    Ok((
        i,
        HandlerBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            handler_type,
            name,
        },
    ))
}
/// Parses a `"pitm"` box from a given generic Box
pub fn parse_primary_item_box(base_box: GenericBox) -> IResult<&[u8], PrimaryItemBox> {
    let (_, full_box) = parse_full_box(base_box)?;

    // Taken from this implementation
    // https://github.com/frejoel/bmffparse/blob/c635e6f6efcccdf4782e53b10dda13c4611507cf/src/parse.c#L521

    let i = full_box.data;
    let item_id: u32;
    if full_box.version == 0 {
        let (_, (item_id_16, _)) = (be_u16, eof).parse(full_box.data)?;
        item_id = item_id_16 as u32;
    } else {
        (_, (item_id, _)) = (be_u32, eof).parse(full_box.data)?;
    }

    Ok((
        i,
        PrimaryItemBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            item_id,
        },
    ))
}
/// Parses a `"meta"` box from a given generic Box
pub fn parse_data_information_box(base_box: GenericBox) -> IResult<&[u8], MetaBox> {
    let (_, full_box) = parse_full_box(base_box)?;

    let (i, child_boxes) = parse_boxes(full_box.data)?;

    Ok((
        i,
        MetaBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            boxes: child_boxes,
        },
    ))
}
/// Parses a `"meta"` box from a given generic Box
pub fn parse_item_location_box(base_box: GenericBox) -> IResult<&[u8], MetaBox> {
    let (_, full_box) = parse_full_box(base_box)?;

    let (i, child_boxes) = parse_boxes(full_box.data)?;

    Ok((
        i,
        MetaBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            boxes: child_boxes,
        },
    ))
}
/// Parses a `"meta"` box from a given generic Box
pub fn parse_item_protection_box(base_box: GenericBox) -> IResult<&[u8], MetaBox> {
    let (_, full_box) = parse_full_box(base_box)?;

    let (i, child_boxes) = parse_boxes(full_box.data)?;

    Ok((
        i,
        MetaBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            boxes: child_boxes,
        },
    ))
}
/// Parses a `"meta"` box from a given generic Box
pub fn parse_item_info_box(base_box: GenericBox) -> IResult<&[u8], MetaBox> {
    let (_, full_box) = parse_full_box(base_box)?;

    let (i, child_boxes) = parse_boxes(full_box.data)?;

    Ok((
        i,
        MetaBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            boxes: child_boxes,
        },
    ))
}

/// Parses a `"meta"` box from a given generic Box
pub fn parse_ipmp_control_box(base_box: GenericBox) -> IResult<&[u8], MetaBox> {
    let (_, full_box) = parse_full_box(base_box)?;

    let (i, child_boxes) = parse_boxes(full_box.data)?;

    Ok((
        i,
        MetaBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            boxes: child_boxes,
        },
    ))
}

/// Parses a `"meta"` box from a given generic Box
pub fn parse_item_reference_box(base_box: GenericBox) -> IResult<&[u8], MetaBox> {
    let (_, full_box) = parse_full_box(base_box)?;

    let (i, child_boxes) = parse_boxes(full_box.data)?;

    Ok((
        i,
        MetaBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            boxes: child_boxes,
        },
    ))
}

/// Parses a `"meta"` box from a given generic Box
pub fn parse_item_data_box(base_box: GenericBox) -> IResult<&[u8], MetaBox> {
    let (_, full_box) = parse_full_box(base_box)?;

    let (i, child_boxes) = parse_boxes(full_box.data)?;

    Ok((
        i,
        MetaBox {
            size: full_box.size,
            box_type: full_box.box_type,
            version: full_box.version,
            flags: full_box.flags,
            boxes: child_boxes,
        },
    ))
}
