use heif_macros::define_box;

// BaseBox + FullBox
#[define_box("ipro", full)]

pub struct ItemProtectionBox<'a> {
    pub protection_count: u16,
    pub protection_info: Vec<ProtectionSchemeInfoBox<'a>>,
}

// BaseBox
#[define_box("sinf")]
pub struct ProtectionSchemeInfoBox<'a> {
    pub data: &'a [u8],
}

#[derive(Debug)]
pub struct Extent {
    pub index: u64,
    pub offset: u64,
    pub length: u64,
}

#[define_box("frma")]
pub struct OriginalFormatBox<'a> {
    /// 4 * u8
    pub data_format: &'a str,
}
