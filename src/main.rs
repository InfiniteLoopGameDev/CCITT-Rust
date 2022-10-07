mod bitbuffer;
mod modecodes;
mod ccittmodes;

fn main() {
    println!("{:?}", ccittmodes::get_modes());
}
