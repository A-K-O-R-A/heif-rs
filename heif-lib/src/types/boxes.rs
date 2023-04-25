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

struct ContainerBox<'a> {
    box_data: BaseBox<'a>,
    child_count: u32,
    children: &'a [BaseBox<'a>],
}

pub enum BoxType {}

pub struct FileTypeBox {}

// https://nokiatech.github.io/heif/technical.html
// Table VII. Image Properties
pub struct ItemPropertyContainerBox {}

pub enum Roles {
    /// A representative image of the image items and image sequence tracks of the file.
    CoverImage,
    /// A smaller-resolution representation of a master image.
    ThumbnaulImage,
    /// An image that complements a master image. For example, an alpha plane or a depth map.
    AuxiliaryImage,
    /// An image that is not a thumbnail image or an auxiliary image. Typically represents a full-resolution displayable image.
    MasterImage,
    /// An image that should never be displayed. Can be present in the file for example as an input image for a derived image.
    HiddenImage,
    /// A coded image that has been derived from other images.
    PreDerivedCodedImage,
    /// A coded representation of an image.
    CodedImage,
    /// An image that is represented in a file by an indicated operation to indicated input images and can be obtained by performing the indicated operation to the indicated input images.
    DerivedImage,
}
