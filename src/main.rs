use std::fs;

mod bitbuffer;
mod ccittcodes;
mod ccittdecode;
mod ccittmodes;
mod modecodes;

fn main() {
    let data = fs::read("./frame2.bin").unwrap();
    let width = data[0].clone() as usize;
    let mut img_data = data.clone();
    img_data.remove(0);
    let mut decoder = ccittdecode::Decoder::new(width, img_data);
    println!("WOO, it compiled {:?}", decoder.decode());
}
