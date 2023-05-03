use super::{generic::*, image::*, meta::*};

#[derive(Debug)]
pub enum ParsedBox<'a> {
    Box(GenericBox<'a>),
    FullBox(GenericFullBox<'a>),
    MetaBox(MetaBox<'a>),
    HandlerBox(HandlerBox<'a>),
    PrimaryItemBox(PrimaryItemBox<'a>),
    ItemLocationBox(ItemLocationBox<'a>),
    ItemInfoBox(ItemInfoBox<'a>),
    ItemInfoEntry(ItemInfoEntry<'a>),
    ItemReferenceBox(ItemReferenceBox<'a>),
    SingleItemTypeReferenceBox(SingleItemTypeReferenceBox<'a>),
    FileTypeBox(FileTypeBox<'a>),
    MediaDataBox(MediaDataBox<'a>),
}
