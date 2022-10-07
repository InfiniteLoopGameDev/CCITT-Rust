#[derive(Debug)]
pub struct ModeCode{
    bits_used: u8,
    mask: u8,
    value: u8,
    r#type: u8
}

pub const MODE_CODES: [u8; 30] = [
	0x1, 4, 1,
	0x1, 3, 2,
	0x1, 1, 3, // 1
	0x03, 3, 4, // 011
	0x03, 6, 5, // 0000 11
	0x03, 7, 6, // 0000 011
	0x2, 3, 7, // 010
	0x02, 6, 8, // 0000 10
	0x02, 7, 9, // 0000 010
	0x01, 7, 10, // 0000 010
];

impl ModeCode{
    pub fn get_vertical_offset(&self) -> i8{
        match self.r#type {
            crate::modecodes::VERTICALZERO => return 0,
            crate::modecodes::VERTICALL1 => return -1,
            crate::modecodes::VERTICALR1 => return 1,
            crate::modecodes::VERTICALL2 => return -2,
            crate::modecodes::VERTICALR2 => return 2,
            crate::modecodes::VERTICALL3 => return -3,
            crate::modecodes::VERTICALR3 => return 3,
            _ => return 0
        }
    }

    pub fn matches(&self, data: u8) -> bool{
        return data & self.mask == self.value
    }

    pub fn new() -> ModeCode{
        return ModeCode{bits_used: 0,
                        mask: 0,
                        value: 0,
                        r#type: 0}
    }
}

impl Clone for ModeCode {
    fn clone(&self) -> ModeCode {
        ModeCode{bits_used: self.bits_used,
                  mask: self.mask,
                  value: self.value,
                  r#type: self.r#type}
    }
}

impl Copy for ModeCode {}

pub fn get_modes() -> [ModeCode; 10]{
    let mut modes = [ModeCode::new(); MODE_CODES.len()/3];

    for i in 0..(MODE_CODES.len()/3) {
        let code = &mut modes[i];
        code.bits_used = MODE_CODES[i*3+1] as u8;
        code.value = MODE_CODES[i*3] << (8 - &code.bits_used);
        code.mask = 0xff << (8 - &code.bits_used);
        code.r#type = MODE_CODES[i*3+2] as u8;
    }

    return modes;
}