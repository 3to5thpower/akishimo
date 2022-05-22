use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub enum BoxType {
    Ftyp,
    Pdin,
    Moov,
    Mvhd,
    Trak,
    Tkhd,
    Tref,
    Edts,
    Elst,
    Mdia,
    Mdhd,
    Minf,
    Stbl,
    Stco,
    Mvex,
    Ipmc,
    Moof,
    Mfhd,
    Traf,
    Mfra,
    Mdat,
    Free,
    Skip,
    Udta,
    Meta,
    Others(String),
    Error,
}
impl BoxType {
    pub fn from(bytes: &[u8]) -> BoxType {
        if let Ok(typ) = std::str::from_utf8(bytes) {
            match typ {
                "ftyp" => BoxType::Ftyp,
                "pdin" => BoxType::Pdin,
                "moov" => BoxType::Moov,
                "mvhd" => BoxType::Mvhd,
                "trak" => BoxType::Trak,
                "tkhd" => BoxType::Tkhd,
                "tref" => BoxType::Tref,
                "edts" => BoxType::Edts,
                "elst" => BoxType::Elst,
                "mdia" => BoxType::Mdia,
                "mdhd" => BoxType::Mdhd,
                "minf" => BoxType::Minf,
                "stbl" => BoxType::Stbl,
                "stco" => BoxType::Stco,
                "mvex" => BoxType::Mvex,
                "ipmc" => BoxType::Ipmc,
                "moof" => BoxType::Moof,
                "mfhd" => BoxType::Mfhd,
                "traf" => BoxType::Traf,
                "mfra" => BoxType::Mfra,
                "mdat" => BoxType::Mdat,
                "free" => BoxType::Free,
                "skip" => BoxType::Skip,
                "udta" => BoxType::Udta,
                "meta" => BoxType::Meta,
                s => BoxType::Others(s.to_owned()),
            }
        } else {
            BoxType::Error
        }
    }
    pub fn is_leaf(&self) -> bool {
        use BoxType::*;
        match &self {
            Moov | Trak | Edts | Mdia | Minf | Stbl | Moof | Skip => false,
            _ => true,
        }
    }
    pub fn to_string(&self) -> String {
        use BoxType::*;
        if let Others(s) = self {
            return s.to_owned();
        }

        match *self {
            Ftyp => "ftyp",
            Pdin => "pdin",
            Moov => "moov",
            Mvhd => "mvhd",
            Trak => "trak",
            Tkhd => "tkhd",
            Tref => "tref",
            Edts => "edts",
            Elst => "elst",
            Mdia => "mdia",
            Mdhd => "mdhd",
            Minf => "minf",
            Stbl => "stbl",
            Stco => "stco",
            Mvex => "mvex",
            Ipmc => "ipmc",
            Moof => "moof",
            Mfhd => "mfhd",
            Traf => "traf",
            Mfra => "mfra",
            Mdat => "mdat",
            Free => "free",
            Skip => "skip",
            Udta => "udta",
            Meta => "meta",
            Error => "error",
            _ => unreachable!(),
        }
        .to_owned()
    }
}

#[derive(Debug)]
pub struct Mp4Container {
    pub size: u64,
    pub box_type: BoxType,
    pub children: Vec<Mp4Box>,
}
