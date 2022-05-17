#[derive(Debug)]
pub enum Mp4Box {
    Leaf(Mp4Leaf),
    Node(Mp4Container),
}

#[derive(Debug)]
pub struct Mp4Leaf {
    size: usize,
    box_type: BoxType,
    data: Vec<u8>,
}

#[derive(Debug)]
pub enum BoxType {
    ftyp,
    mdat,
    moov,
    stbl,
    others,
    error,
}
impl BoxType {
    pub fn from(bytes: &[u8]) -> BoxType {
        if let Ok(typ) = std::str::from_utf8(bytes) {
            match typ {
                "ftyp" => BoxType::ftyp,
                "mdat" => BoxType::mdat,
                "moov" => BoxType::moov,
                "stbl" => BoxType::stbl,
                _ => BoxType::others,
            }
        } else {
            BoxType::error
        }
    }
}

#[derive(Debug)]
pub struct Mp4Container {
    size: usize,
    box_type: BoxType,
    children: Vec<Mp4Box>,
}
