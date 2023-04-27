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
    pub items: ItemLocation,
}

pub struct ItemLocation {
    pub item_id: u32,
    pub construction_method: u8,
    pub data_reference_index: u16,
    pub base_offset: u64,
    pub extent_count: u16,
    pub extents: Vec<Extent>,
}

pub struct Extent {
    pub index: u64,
    pub offset: u64,
    pub length: u64,
}
