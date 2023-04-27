use super::{base::*, image::*, meta::*};

pub enum GenericBox<'a> {
    Box(BaseBox<'a>),
    FullBox(FullBox<'a>),
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
