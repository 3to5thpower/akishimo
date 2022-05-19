use crate::mp4_body::{BoxType, Mp4Box, Mp4Leaf};
use anyhow::{anyhow, Result};

pub fn parse(content: &[u8]) -> Result<Mp4Box> {
    if content.len() < 16 {
        return Err(anyhow!("Invalid content: {:?}", content));
    }

    let (size, data_start) = match u32::from_be_bytes(content[0..4].try_into().unwrap()) {
        1 => (u32::from_be_bytes(content[8..16].try_into().unwrap()), 16),
        size => (size, 8),
    };

    let box_type = BoxType::from(&content[4..8]);

    let data = content[data_start as usize..(data_start + size) as usize].to_vec();

    Ok(Mp4Box::Leaf(Mp4Leaf {
        size,
        box_type,
        data,
    }))
}

fn is_leaf(content: &[u8]) -> Result<bool> {
    if content.len() < 16 {
        return Err(anyhow!("Invalid content: {:?}", content));
    }
    unimplemented!()
}
