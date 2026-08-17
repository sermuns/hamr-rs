use strum::FromRepr;

use crate::compress::Segment;

#[derive(FromRepr, PartialEq)]
pub enum SegmentType {
    Path,
    Query,
    Hash,
}

impl From<&Segment<'_>> for SegmentType {
    // TODO: seems verbose and stupid...
    fn from(value: &Segment) -> Self {
        match value {
            Segment::Query { .. } => SegmentType::Query,
            Segment::Hash(..) => SegmentType::Hash,
            Segment::Path(..) => SegmentType::Path,
        }
    }
}
