use crate::boxes::generic::GenericBox;

extern crate heif_macros;
use heif_macros::define_box;

#[define_box("moov", container)]
pub struct MovieBox<'a> {}

#[define_box("moof", container)]
pub struct MovieFragmentBox<'a> {}

#[define_box("mfra", container)]
pub struct MovieFragmentRandomAccessBox<'a> {}

#[define_box("dinf", container)]
pub struct DataInformationBox<'a> {}

#[define_box("trak", container)]
pub struct TrackBox<'a> {}

#[define_box("mvex", container)]
pub struct MovieExtendsBox<'a> {}

#[define_box("traf", container)]
pub struct TrackFragmentBox<'a> {}

#[define_box("udta", container)]
pub struct UserDataBox<'a> {}

#[define_box("edts", container)]
pub struct EditBox<'a> {}

#[define_box("mdia", container)]
pub struct MediaBox<'a> {}

#[define_box("minf", container)]
pub struct MediaInformationBox<'a> {}

#[define_box("stbl", container)]
pub struct SampleTableBox<'a> {}

#[define_box("meco", container)]
pub struct AdditionalMetadataContainerBox<'a> {}

#[define_box("strk", container)]
pub struct SubTrackBox<'a> {}

#[define_box("strd", container)]
pub struct SubTrackDefinitionBox<'a> {}

#[define_box("schi", container)]
pub struct SchemeInformationBox<'a> {}
