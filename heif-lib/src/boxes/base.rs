use std::fmt;

// All Boxes contain the base Box as the first item in the structure.
// Length - Type - Value
#[derive(Debug, Clone, Copy)]
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
            "Box {{\n    size: {},\n    type: {},\n    data: [...]\n}}",
            self.size, self.box_type
        )
    }
}

#[derive(Debug)]
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
            "FullBox {{\n    size: {},\n    type: {},\n    version: {:08b},\n    flags: {:024b},\n    data: [...]\n}}",
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
