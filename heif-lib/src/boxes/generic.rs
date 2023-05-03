use std::fmt::{self};

// All Boxes contain the base Box as the first item in the structure.
// Length - Type - Value
#[derive(Clone, Copy)]
pub struct GenericBox<'a> {
    pub size: u32,
    // 4 bytes long string
    pub box_type: &'a str,
    pub data: &'a [u8],
}

impl<'a> fmt::Display for GenericBox<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Box {{ size: {}, type: {:?}, data: [...] }}",
            self.size, self.box_type
        )
    }
}

impl<'a> fmt::Debug for GenericBox<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Box {{ size: {}, type: {:?}, data: [...] }}",
            self.size, self.box_type
        )
    }
}

pub struct GenericFullBox<'a> {
    pub size: u32,
    // Length 4
    pub box_type: &'a str,
    pub version: u8,
    /// only 24 bits -> 3 bytes
    pub flags: u32,
    pub data: &'a [u8],
}

impl<'a> fmt::Display for GenericFullBox<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FullBox {{ size: {}, type: {:?}, version: {:08b}, flags: {:024b}, data: [...] }}",
            self.size, self.box_type, self.version, self.flags
        )
    }
}

impl<'a> fmt::Debug for GenericFullBox<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FullBox {{ size: {}, type: {:?}, version: {:08b}, flags: {:024b}, data: [...] }}",
            self.size, self.box_type, self.version, self.flags
        )
    }
}

#[derive(Debug)]
pub struct GenericContainerBox<'a> {
    pub size: u32,
    pub box_type: &'a str,
    pub children: Vec<GenericBox<'a>>,
}
