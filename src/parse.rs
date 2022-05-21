use crate::mp4_body::{BoxType, Mp4Box, Mp4Container, Mp4Leaf};
use anyhow::{anyhow, Result};

pub fn parse(content: &[u8]) -> Result<Vec<Mp4Box>> {
    parse_children(content)
}

fn parse_box(content: &[u8]) -> Result<Mp4Box> {
    let (size, data_start) = read_box_size_and_data_start(content)?;
    let box_type = BoxType::from(&content[4..8]);

    if box_type.is_leaf() {
        let data = content[data_start as usize..(data_start + size) as usize].to_vec();
        Ok(Mp4Box::Leaf(Mp4Leaf {
            size,
            box_type,
            data,
        }))
    } else {
        Ok(Mp4Box::Node(Mp4Container {
            size,
            box_type,
            children: parse_children(&content[data_start as usize..])?,
        }))
    }
}

/// BOXのサイズとdataの開始部分までのオフセットのタプルを返します
fn read_box_size_and_data_start(data: &[u8]) -> Result<(u64, u64)> {
    if data.len() < 16 {
        return Err(anyhow!("Invalid content: {:?}", data));
    }


    let (size, data_start) = match u32::from_be_bytes(data[0..4].try_into().unwrap()) {
        1 => (u64::from_be_bytes(data[8..16].try_into().unwrap()), 16),
        size => (size as u64, 8),
    };
    Ok((size, data_start))
}

/// BOXの子要素になっている連続したBOXをパースします
/// * `data` - 子要素部分のバイト列
/// * `data_size` - 子要素部分のみのサイズ
fn parse_children(data: &[u8]) -> Result<Vec<Mp4Box>> {
    let mut children = vec![];

    let mut data_start: usize = 0;
    let mut content_slice = data;

    while let Ok((size, _)) = read_box_size_and_data_start(content_slice) {
        let size = size as usize;
        children.push(parse_box(content_slice)?);
        content_slice = &content_slice[data_start + size..];
        data_start += size;
    }

    Ok(children)
}
