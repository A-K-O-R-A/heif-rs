// BaseBox + FullBox
pub struct ItemProtectionBox<'a> {
    pub size: u32,
    /// This will always be `"ipro"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    // end of full box
    pub protection_count: u16,
    pub protection_info: Vec<ProtectionSchemeInfoBox<'a>>,
}

// BaseBox
pub struct ProtectionSchemeInfoBox<'a> {
    pub size: u32,
    /// This will always be `"sinf"`
    pub box_type: &'a str,
}

pub struct Extent {
    pub index: u64,
    pub offset: u64,
    pub length: u64,
}

pub struct OriginalFormatBox<'a> {
    pub size: u32,
    /// This will always be `"frma"`
    pub box_type: &'a str,
    /// 4 * u8
    pub data_format: &'a str,
}
