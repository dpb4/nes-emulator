use std::{env, fs};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    if env::args().collect::<Vec<String>>().len() < 2 {
        println!("missing rom name to execute");
        return;
    }
    let rom_name: &String = &env::args().collect::<Vec<String>>()[1];

    let raw_bytes = fs::read(rom_name.to_owned() + &".nes".to_string()).unwrap();
    nes_emulator::start(raw_bytes);
}
