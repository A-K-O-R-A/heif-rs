#[derive(Debug)]
pub struct FileTypeBox<'a> {
    pub size: u32,
    /// This will always be `"ftyp"`
    pub box_type: &'a str,
    pub major_brand: &'a str,
    pub minor_version: u32,
    /// Each brand is 4 bytes in long
    pub compatible_brands: Vec<&'a str>,
}

#[derive(Debug)]
pub struct MediaDataBox<'a> {
    pub size: u32,
    /// This will always be `"mdat"`
    pub box_type: &'a str,
    pub data: &'a [u8],
}
