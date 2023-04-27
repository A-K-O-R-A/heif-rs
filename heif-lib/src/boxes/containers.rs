use crate::boxes::base::BaseBox;

#[derive(Debug)]
pub struct ContainerBox<'a> {
    pub size: u32,
    pub box_type: &'a str,
    pub children: Vec<BaseBox<'a>>,
}

/// Box type `"moov"`
pub type MovieBox<'a> = ContainerBox<'a>;

/// Box type `"moof"`
pub type MovieFragmentBox<'a> = ContainerBox<'a>;

/// Box type `"mfra"`
pub type MovieFragmentRandomAccessBox<'a> = ContainerBox<'a>;

/// Box type `"dinf"`
pub type DataInformationBox<'a> = ContainerBox<'a>;

/// Box type `"trak"`
pub type TrackBox<'a> = ContainerBox<'a>;

/// Box type `"mvex"`
pub type MovieExtendsBox<'a> = ContainerBox<'a>;

/// Box type `"traf"`
pub type TrackFragmentBox<'a> = ContainerBox<'a>;

/// Box type `"udta"`
pub type UserDataBox<'a> = ContainerBox<'a>;

/// Box type `"edts"`
pub type EditBox<'a> = ContainerBox<'a>;

/// Box type `"mdia"`
pub type MediaBox<'a> = ContainerBox<'a>;

/// Box type `"minf"`
pub type MediaInformationBox<'a> = ContainerBox<'a>;

/// Box type `"stbl"`
pub type SampleTableBox<'a> = ContainerBox<'a>;

/// Box type `"meco"`
pub type AdditionalMetadataContainerBox<'a> = ContainerBox<'a>;

/// Box type `"strk"`
pub type SubTrackBox<'a> = ContainerBox<'a>;

/// Box type `"strd"`
pub type SubTrackDefinitionBox<'a> = ContainerBox<'a>;

/// Box type `"schi"`
pub type SchemeInformationBox<'a> = ContainerBox<'a>;
