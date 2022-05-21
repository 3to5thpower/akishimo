#[derive(Debug)]
pub enum Mp4Box {
    Leaf(Mp4Leaf),
    Node(Mp4Container),
}

#[derive(Debug)]
pub struct Mp4Leaf {
    pub size: u64,
    pub box_type: BoxType,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum BoxType {
    Ftyp,
    Mdat,
    Moov,
    Stbl,
    Others,
    Error,
}
impl BoxType {
    pub fn from(bytes: &[u8]) -> BoxType {
        if let Ok(typ) = std::str::from_utf8(bytes) {
            match typ {
                "ftyp" => BoxType::Ftyp,
                "mdat" => BoxType::Mdat,
                "moov" => BoxType::Moov,
                "stbl" => BoxType::Stbl,
                _ => BoxType::Others,
            }
        } else {
            BoxType::Error
        }
    }
    pub fn is_leaf(&self) -> bool {
        use BoxType::*;
        match &self {
            Mdat => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Mp4Container {
    pub size: u64,
    pub box_type: BoxType,
    pub children: Vec<Mp4Box>,
}
