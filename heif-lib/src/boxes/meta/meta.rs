use heif_macros::define_box;

use crate::boxes::generic::{GenericBox, GenericFullBox};

// BaseBox + FullBox
#[derive(Debug)]
pub struct MetaBox<'a> {
    pub size: u32,
    /// This will always be `"meta"`
    pub box_type: &'a str,
    pub version: u8,
    /// only 24 bits -> 3 bytes
    pub flags: u32,
    // End of full box
    pub boxes: Vec<GenericBox<'a>>,
}

// BaseBox + FullBox
#[define_box("hdlr", full)]
pub struct HandlerBox<'a> {
    pub handler_type: &'a [u8; 4],
    // Unsure if this is a string
    pub name: &'a str,
}

// BaseBox + FullBox
#[define_box("pitm", full)]
pub struct PrimaryItemBox<'a> {
    // End of FullBox
    pub item_id: u32,
}

// BaseBox + FullBox
#[define_box("iinf", full)]
pub struct ItemInfoBox<'a> {
    pub entry_count: u32,
    pub entries: Vec<GenericFullBox<'a>>,
}

// BaseBox + FullBox
#[define_box("infe", full)]
pub struct ItemInfoEntry<'a> {
    pub item_id: u32,
    pub item_protection_index: u32,
    pub item_name: &'a str,
    pub content_type: u32,
    pub content_encoding: Option<String>,
    pub extension_type: Option<[u8; 4]>,
    // Still missing fields
}
// BaseBox + FullBox
#[define_box("iref", full)]
pub struct ItemReferenceBox<'a> {
    pub references_count: u32,
    // SingleItemTypeReferenceBox
    pub references: Vec<SingleItemTypeReferenceBox<'a>>,
}

// BaseBox
#[define_box("????")]
pub struct SingleItemTypeReferenceBox<'a> {
    pub from_item_id: u32,
    pub reference_count: u16,
    pub to_item_ids: Vec<u32>,
}

// BaseBox + FullBox
#[define_box("idat", full)]
pub struct ItemDataBox<'a> {
    pub entries: Vec<GenericFullBox<'a>>,
}
