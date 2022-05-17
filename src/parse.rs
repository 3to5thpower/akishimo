use crate::mp4_body::{BoxType, Mp4Box};
use anyhow::{anyhow, bail, Result};

pub fn parse(content: &[u8]) -> Result<Mp4Box> {
    if content.len() < 16 {
        return Err(anyhow!("Invalid content: {:?}", content));
    }

    let (size, data_start) = match u32::from_be_bytes(content[0..4].try_into().unwrap()) {
        1 => (u32::from_be_bytes(content[8..16].try_into().unwrap()), 16),
        size => (size, 8),
    };

    let typ = BoxType::from(&content[4..8]);

    let data = content[data_start as usize..(data_start + size) as usize].to_vec();

    Ok(Mp4Box { size, typ, data })
}
