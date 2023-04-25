use nom::IResult;

use crate::boxes::base::BaseBox;
use crate::boxes::meta::MetaBox;

use super::base_box::{parse_boxes, parse_full_box};

pub fn parse_meta_box(base_box: BaseBox) -> IResult<&[u8], MetaBox> {
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

pub fn parse_handler_box(base_box: BaseBox) -> IResult<&[u8], MetaBox> {
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
