#[derive(Copy, Clone)]
pub struct IhdrChunck {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub zmethod: u8, //compressing method
    pub fmethod: u8, //filtering method
    pub icmethod: u8 //interlaced scan method
}

impl IhdrChunck {
    pub fn check(self) -> Result<(), String> {
        if self.zmethod != 0 || self.fmethod != 0 {
            return Err("Invalid method".to_string());
        }
        else if self.color_type != 6 || self.bit_depth != 8 || self.icmethod != 0 {
            return Err("Not supported".to_string());
        }
        Ok(())
    }

    pub fn from_bytes(bytes: Vec<u8>) -> IhdrChunck {
        let slice = bytes.as_slice().clone();
        Self {
            width: u32::from_be_bytes([
                slice[0].clone(),
                slice[1].clone(),
                slice[2].clone(),
                slice[3].clone()
            ]),
            height: u32::from_be_bytes([
                slice[4].clone(),
                slice[5].clone(),
                slice[6].clone(),
                slice[7].clone()
            ]),
            bit_depth: slice[8],
            color_type: slice[9],
            zmethod: slice[10],
            fmethod: slice[11],
            icmethod: slice[12]
        }
    }
}