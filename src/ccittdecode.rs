use crate::modecodes as modes;

fn reverse_color(current: u8) -> u8 {
    if current == 0 {
        return 255;
    }
    0
}

fn end_of_block(buffer: u32) -> bool {
    (buffer & 0xffffff00) == 0x00100100
}

fn get_previous_line(lines: &[Vec<u8>], current_line: usize, width: usize) -> Vec<u8> {
    if current_line == 0 {
        let white_out = vec![255; width];
        return white_out;
    }
    lines[current_line - 1].clone()
}

fn find_b_values(ref_line: Vec<u8>, a0_pos: usize, a0_color: u8, justb1: bool) -> (usize, usize) {
    let other = reverse_color(a0_color);
    let mut start_pos = a0_pos;
    if start_pos != 0 {
        start_pos += 1;
    }
    let (mut b1, mut b2) = (0, 0);

    for i in start_pos..ref_line.len() {
        let cur_color: u8 = if i == 0 {ref_line[0]} else { ref_line[i] };
        let last_color: u8 = if i == 0 {255} else { ref_line[i - 1] };

        if b1 != 0 && cur_color == a0_color && last_color == other {
            b2 = i;
            return (b1, b2);
        }

        if cur_color == other && last_color == a0_color {
            b1 = i;
            if b2 != 0 || justb1 {
                return (b1, b2);
            }
        }
    }
    if b1 == 0 {
        b1 = ref_line.len()
    } else {
        b2 = ref_line.len()
    }

    (b1, b2)
}

pub struct Decoder {
    pub reverse_color: bool,
    width: usize,
    buffer: crate::bitbuffer::BitBuffer,
    mode_codes: [crate::ccittmodes::ModeCode; 10],
    horizontal_codes: crate::ccittcodes::HorizontalCodes,
}

impl Decoder {
    pub fn new(width: usize, bytes: Vec<u8>) -> Decoder {
        Decoder {
            reverse_color: false,
            width,
            horizontal_codes: crate::ccittcodes::HorizontalCodes::new(),
            mode_codes: crate::ccittmodes::get_modes(),
            buffer: crate::bitbuffer::BitBuffer::new(bytes),
        }
    }

    fn get_mode(&self) -> crate::ccittmodes::ModeCode {
        let r#match = crate::ccittmodes::ModeCode::new();
        let (b8, _) = self.buffer.peak_8();
        for i in self.mode_codes {
            if i.matches(b8) {
                return i;
            }
        }
        r#match
    }

    pub fn decode(&mut self) -> Vec<Vec<u8>> {
        let mut lines: Vec<Vec<u8>> = Vec::new();
        let mut line: Vec<u8> = vec![0; self.width];
        let mut line_pos = 0;
        let mut cur_line = 0;
        let mut a0_color: u8 = 255;

        while self.buffer.has_data() {
            if line_pos > self.width as usize - 1 {
                lines.push(line.clone());
                line = vec![0; self.width];
                line_pos = 0;
                a0_color = 255;
                cur_line += 1;
                if end_of_block(self.buffer.buffer) {
                    break;
                }
            }

            let (v, _) = self.buffer.peak_32();

            if v == 0x00000000 {
                break;
            }

            let mode = self.get_mode();
            self.buffer.flush_bits(mode.bits_used);

            match mode.r#type {
                modes::PASS => {
                    let (_, b2) = find_b_values(
                        get_previous_line(&lines, cur_line, self.width),
                        line_pos,
                        a0_color,
                        false,
                    );

                    for p in 0..(b2-line_pos) {
                        line[line_pos + p] = a0_color;
                    }
                    line_pos += b2-line_pos;
                }
                modes::HORIZONTAL => {
                    let mut is_white = a0_color == 0xff;
                    let mut length = [0u16, 0];
                    let mut color = [127u8, 127];

                    for i in 0..2 {
                        let mut scan = true;
                        while scan {
                            let h = self
                                .horizontal_codes
                                .find_match_32(self.buffer.buffer, is_white);
                            self.buffer.flush_bits(h.bits_used);
                            length[i] += h.pixels;
                            color[i] = h.color as u8;
                            if h.terminating {
                                is_white = !is_white;
                                scan = false;
                            }
                        }
                    }

                    let mut pixel_length: usize;

                    for i in 0..2 {
                        pixel_length = length[i] as usize;
                        for _p in 0..pixel_length {
                            if line_pos < line.len() {
                                line[line_pos] = color[i];
                            }
                            line_pos += 1;
                        }
                    }
                }
                modes::VERTICALZERO
                | modes::VERTICALL1
                | modes::VERTICALR1
                | modes::VERTICALL2
                | modes::VERTICALR2
                | modes::VERTICALL3
                | modes::VERTICALR3 => {
                    let offset = mode.get_vertical_offset() as isize;
                    let (b1, _) = find_b_values(
                        get_previous_line(&lines, cur_line, self.width),
                        line_pos,
                        a0_color,
                        true,
                    );

                    for _i in line_pos as isize..(b1 as isize + offset) as isize {
                        if line_pos < line.len() {
                            line[line_pos] = a0_color
                        }
                        line_pos += 1
                    }

                    a0_color = reverse_color(a0_color)
                }
                _ => return lines,
            }
        }

        if self.reverse_color {
            for i in 0..lines.len() {
                for x in 0..lines[i].len() {
                    lines[i][x] = reverse_color(lines[i][x]);
                }
            }
        }

        lines
    }
}
