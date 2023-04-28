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
pub struct HandlerBox<'a> {
    pub size: u32,
    /// This will always be `"hdlr"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    // End of full box
    pub handler_type: &'a [u8; 4],
    // Unsure if this is a string
    pub name: &'a str,
}

// BaseBox + FullBox
pub struct PrimaryItemBox<'a> {
    pub size: u32,
    /// This will always be `"pitm"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    // End of FullBox
    pub item_id: u32,
}

// BaseBox + FullBox
pub struct ItemInfoBox<'a> {
    pub size: u32,
    /// This will always be `"iinf"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    pub entry_count: u32,
    pub entries: Vec<GenericFullBox<'a>>,
}

// BaseBox + FullBox
pub struct ItemInfoEntry<'a> {
    pub size: u32,
    /// This will always be `"iinf"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    //
    pub item_id: u32,
    pub item_protection_index: u32,
    pub item_name: &'a str,
    pub content_type: u32,
    pub content_encoding: Option<String>,
    pub extension_type: Option<[u8; 4]>,
    // Still missing fields
}
// BaseBox + FullBox
pub struct ItemReferenceBox<'a> {
    pub size: u32,
    /// This will always be `"iref"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    pub references_count: u32,
    // SingleItemTypeReferenceBox
    pub references: Vec<SingleItemTypeReferenceBox<'a>>,
}

// BaseBox
pub struct SingleItemTypeReferenceBox<'a> {
    pub size: u32,
    /// This will always be `"????"`
    pub box_type: &'a str,
    pub from_item_id: u32,
    pub reference_count: u16,
    pub to_item_ids: Vec<u32>,
}

// BaseBox + FullBox
pub struct ItemDataBox<'a> {
    pub size: u32,
    /// This will always be `"iinf"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    pub entry_count: u32,
    pub entries: Vec<GenericFullBox<'a>>,
}
