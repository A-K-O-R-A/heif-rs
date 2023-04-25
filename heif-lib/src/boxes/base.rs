// All Boxes contain the base Box as the first item in the structure.
// Length - Type - Value
#[derive(Debug)]
pub struct BaseBox<'a> {
    pub size: u32,
    // Length 4
    pub box_type: &'a [u8], // [b'a', b'b', b'c', b'd']
    pub data: &'a [u8],
}

impl<'a> BaseBox<'a> {
    fn box_type(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.box_type)
    }
}

use std::fmt;
impl<'a> fmt::Display for BaseBox<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let box_type = self.box_type().expect("Invalid box type string");
        write!(
            f,
            "Box {{ size: {}, type: {:?}, data: [...] }}",
            self.size,
            box_type //, self.data
        )
    }
}
