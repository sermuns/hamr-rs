pub enum Lookup {
    TldEncode,
    SldEncode,
    DomainEncode,
    PathEncode,
    TldDecode,
    SldDecode,
    DomaiDeecode,
    PathDecode,
}

mod lookup;

pub fn huffman_decode(number: usize, lookup: Lookup) {}

