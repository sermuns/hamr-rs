pub enum Lookup {
    TldEncode,
    SldEncode,
    DomainEncode,
    PathEncode,
    TldDecode,
    SldDecode,
    DomainDecode,
    PathDecode,
}

mod domain;
mod path;
mod sld;
mod tld;

pub fn huffman_decode(number: usize, lookup: Lookup) {
    path::PathEncode::lookup("ok")
}
