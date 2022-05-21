use crate::mp4_body::{BoxType, Mp4Box, Mp4Container, Mp4Leaf};
use anyhow::{anyhow, Result};
use derive_new::new;
use std::fmt;

#[derive(Debug, new)]
pub struct Mp4BoxInfo {
    box_type: BoxType,
    start_index: usize,
    end_index: usize,
    size: usize,
    // parent_box_: Option<>
}
impl fmt::Display for Mp4BoxInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Mp4BoxInfo{box_type, start_index, end_index, size} = self;
        write!(f, "file type `{}`: start is {}, end is {}, size is {}", 
        box_type.to_str(), start_index, end_index, size)
    }
}

pub fn analyzeMp4BoxInfo(content: &[u8]) -> Result<Vec<Mp4BoxInfo>> {
    let mut res = vec![];

    let mut index = 0;
    let mut content_slice = content;
    while let Ok((size, start_index)) = read_box_size_and_data_start(content_slice) {
        let box_type = BoxType::from(&content[index+4..index+8]);
        let end_index = index + size;
        res.push(Mp4BoxInfo::new(box_type, start_index, end_index, size));
        content_slice = &content_slice[end_index..];
        index = end_index;
    }
    Ok(res)
}



// /// BOXの子要素になっている連続したBOXをパースします
// /// * `data` - 子要素部分のバイト列
// pub fn parse(data: &[u8]) -> Result<Vec<Mp4Box>> {
//     let mut children = vec![];

//     let mut data_start: usize = 0;
//     let mut content_slice = data;

//     while let Ok((size, _)) = read_box_size_and_data_start(content_slice) {
//         let size = size as usize;
//         children.push(parse_box(content_slice)?);
//         content_slice = &content_slice[data_start + size..];
//         data_start += size;
//     }

//     Ok(children)
// }

// fn parse_box(content: &[u8]) -> Result<Mp4Box> {
//     let (size, data_start) = read_box_size_and_data_start(content)?;
//     let box_type = BoxType::from(&content[4..8]);

//     if box_type.is_leaf() {
//         let data = content[data_start as usize..(data_start + size) as usize].to_vec();
//         Ok(Mp4Box::Leaf(Mp4Leaf {
//             size,
//             box_type,
//             data,
//         }))
//     } else {
//         Ok(Mp4Box::Node(Mp4Container {
//             size,
//             box_type,
//             children: parse(&content[data_start as usize..])?,
//         }))
//     }
// }

/// BOXのサイズとdataの開始部分までのオフセットのタプルを返します
fn read_box_size_and_data_start(data: &[u8]) -> Result<(usize, usize)> {
    if data.len() < 16 {
        return Err(anyhow!("Invalid content: {:?}", data));
    }


    let (size, data_start) = match usize::from_be_bytes(data[0..4].try_into().unwrap()) {
        1 => (usize::from_be_bytes(data[8..16].try_into().unwrap()), 16),
        size => (size, 8),
    };
    Ok((size, data_start))
}

