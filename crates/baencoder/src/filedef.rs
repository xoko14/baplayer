use binrw::{BinRead, BinWrite};

#[derive(BinRead, BinWrite)]
pub struct BaFile {
    pub frame_height: u8,
    pub frame_width: u8,
    pub frame_rate: u8,
    pub frame_count: u32,
    #[br(count = frame_count)]
    pub frames: Vec<Frame>,
}

#[derive(BinRead, BinWrite)]
pub struct Frame {
    pub section_count: u16,
    #[br(count = section_count)]
    pub sections: Vec<Section>,
}

#[derive(BinRead, BinWrite)]
pub struct Section {
    pub color: u8,
    pub pixel_count: u16,
}

impl Frame {
    pub fn new(sections: Vec<Section>) -> Self {
        Self {
            section_count: sections.len() as u16,
            sections,
        }
    }
}

impl Section {
    pub fn new(color: u8, pixel_count: u16) -> Self {
        Self { color, pixel_count }
    }
}
