use crate::boxes::base::BaseBox;

extern crate heif_macros;
use heif_macros::define_box;

#[derive(Debug)]
pub struct GenericContainerBox<'a> {
    pub size: u32,
    pub box_type: &'a str,
    pub children: Vec<BaseBox<'a>>,
}

/// Box type `"moov"`
pub type MovieBox<'a> = GenericContainerBox<'a>;

/// Box type `"moof"`
pub type MovieFragmentBox<'a> = GenericContainerBox<'a>;

/// Box type `"mfra"`
pub type MovieFragmentRandomAccessBox<'a> = GenericContainerBox<'a>;

/// Box type `"dinf"`
pub type DataInformationBox<'a> = GenericContainerBox<'a>;

/// Box type `"trak"`
pub type TrackBox<'a> = GenericContainerBox<'a>;

/// Box type `"mvex"`
pub type MovieExtendsBox<'a> = GenericContainerBox<'a>;

/// Box type `"traf"`
pub type TrackFragmentBox<'a> = GenericContainerBox<'a>;

/// Box type `"udta"`
pub type UserDataBox<'a> = GenericContainerBox<'a>;

/// Box type `"edts"`
pub type EditBox<'a> = GenericContainerBox<'a>;

/// Box type `"mdia"`
pub type MediaBox<'a> = GenericContainerBox<'a>;

/// Box type `"minf"`
pub type MediaInformationBox<'a> = GenericContainerBox<'a>;

/// Box type `"stbl"`
pub type SampleTableBox<'a> = GenericContainerBox<'a>;

/// Box type `"meco"`
pub type AdditionalMetadataContainerBox<'a> = GenericContainerBox<'a>;

/// Box type `"strk"`
pub type SubTrackBox<'a> = GenericContainerBox<'a>;

/// Box type `"strd"`
pub type SubTrackDefinitionBox<'a> = GenericContainerBox<'a>;

/// Box type `"schi"`
pub type SchemeInformationBox<'a> = GenericContainerBox<'a>;
