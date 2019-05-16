use crate::nes::parser::nes_file::read_nes_rom;

mod nes;

fn main() {
    println!("Hello, world!");
    let path = "roms/hello.nes";
    let cartridge = read_nes_rom(path).unwrap();
    for x in cartridge.prg.iter() {
        print!("{}", x);
    }
}
