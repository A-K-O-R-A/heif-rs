// All Boxes contain the base Box as the first item in the structure.
// Length - Type - Value
#[derive(Debug, Clone, Copy)]
pub struct BaseBox<'a> {
    pub size: u32,
    // Length 4
    pub box_type: &'a str,
    pub data: &'a [u8],
}

use std::fmt;
impl<'a> fmt::Display for BaseBox<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Box {{ size: {}, type: {:?}, data: [...] }}",
            self.size,
            self.box_type //, self.data
        )
    }
}

#[derive(Debug)]
pub struct FullBox<'a> {
    pub size: u32,
    // Length 4
    pub box_type: &'a str,
    pub version: u8,
    /// only 24 bits -> 3 bytes
    pub flags: u32,
    pub data: &'a [u8],
}
