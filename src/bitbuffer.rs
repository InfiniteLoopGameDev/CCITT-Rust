pub struct BitBuffer {
    pub buffer: u32,
    pub empty_bits: u8,
    pub source: Vec<u8>,
    pub source_pos: usize,
}

impl BitBuffer {
    pub fn add_byte(&mut self, source: u8) {
        let pad_right = self.empty_bits - 8;
        let zeroed = self.buffer; // >> (8 + pad_right) << (8 + pad_right);
        self.buffer = zeroed | ((source as u32) << pad_right);
        self.empty_bits -= 8;
    }

    pub fn try_fill_buffer(&mut self) {
        while self.empty_bits > 7 {
            if self.source_pos >= self.source.len() {
                break;
            }
            self.add_byte(self.source[self.source_pos]);
            self.source_pos += 1;
        }
    }

    pub fn flush_bits(&mut self, count: u8) {
        self.buffer <<= count;
        self.empty_bits += count;
        self.try_fill_buffer()
    }

    pub fn peak_8(&self) -> (u8, u8) {
        ((self.buffer >> 24) as u8, 32 - self.empty_bits)
    }

    pub fn peak_32(&self) -> (u32, u8) {
        (self.buffer, 32 - self.empty_bits)
    }

    pub fn has_data(&self) -> bool {
        !(self.empty_bits == 32 && self.source_pos >= self.source.len())
    }

    pub fn new(source: Vec<u8>) -> BitBuffer {
        let mut buffer = BitBuffer {
            empty_bits: 32,
            buffer: 0,
            source,
            source_pos: 0,
        };
        buffer.try_fill_buffer();
        buffer
    }
}
