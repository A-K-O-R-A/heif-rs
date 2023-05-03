use heif_macros::define_box;

// BaseBox + FullBox
#[define_box("iloc", full)]
pub struct ItemLocationBox<'a> {
    pub offset_size: u16,
    pub length_size: u16,
    pub base_offset_size: u16,
    pub index_size: u8,
    pub item_count: u32,
    pub items: ItemLocation,
}

#[derive(Debug)]
pub struct ItemLocation {
    pub item_id: u32,
    pub construction_method: u8,
    pub data_reference_index: u16,
    pub base_offset: u64,
    pub extent_count: u16,
    pub extents: Vec<Extent>,
}

#[derive(Debug)]
pub struct Extent {
    pub index: u64,
    pub offset: u64,
    pub length: u64,
}
