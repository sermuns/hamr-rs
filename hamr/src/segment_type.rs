use strum::FromRepr;

#[derive(FromRepr)]
pub enum SegmentType {
    Path,
    Query,
    Hash,
}
