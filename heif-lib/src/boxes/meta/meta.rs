use crate::boxes::base::{BaseBox, FullBox};

// BaseBox + FullBox
#[derive(Debug)]
pub struct MetaBox<'a> {
    pub size: u32,
    /// This will always be `"meta"`
    pub box_type: &'a str,
    pub version: u8,
    /// only 24 bits -> 3 bytes
    pub flags: u32,
    pub boxes: Vec<BaseBox<'a>>,
}

// BaseBox + FullBox
pub struct HandlerBox<'a> {
    pub size: u32,
    /// This will always be `"hdlr"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
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
    pub item_id: u32,
}

// BaseBox + FullBox
pub struct ItemLocationBox<'a> {
    pub size: u32,
    /// This will always be `"iloc"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    pub offset_size: u16,
    pub length_size: u16,
    pub base_offset_size: u16,
    pub index_size: u8,
    pub item_count: u32,
    // + Item Locations
}

// BaseBox + FullBox
pub struct ItemInfoBox<'a> {
    pub size: u32,
    /// This will always be `"iinf"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    pub entry_count: u32,
    pub entries: Vec<FullBox<'a>>,
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
pub struct IPMPControlBox<'a> {
    pub size: u32,
    /// This will always be `"iinf"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    // End of full box
    pub tool_list: IPMPToolListDescriptor<'a>,
    pub ipmp_descriptors: Vec<IPMPDescriptor<'a>>,
}

// BaseBox + FullBox
pub struct IPMPToolListDescriptor<'a> {
    pub size: u32,
    /// This will always be `"iinf"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    // End of full box
    pub entry_count: u32,
    pub entries: Vec<FullBox<'a>>,
}

// BaseBox + FullBox
pub struct IPMPDescriptor<'a> {
    pub size: u32,
    /// This will always be `"iinf"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    // End of full box
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
    pub entries: Vec<FullBox<'a>>,
}
