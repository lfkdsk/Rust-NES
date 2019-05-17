use std::ptr::null;

use bytes::BytesMut;

use crate::nes::parser::nes_file::Cartridge;

use self::cpu::cpu::CPU;
use self::memory::memory::Memory;
use self::parser::nes_file::read_nes_rom;

pub mod cpu;
pub mod memory;
pub mod parser;


pub struct Nes {
    cpu: CPU,
    ram: BytesMut,
    cartridge: Cartridge,
}

impl Nes {
    pub fn create_nes(path: &str) -> Nes {
        let cartridge: Cartridge = read_nes_rom(path).unwrap();
        let mut nes = Nes {
            cpu: null(),
            ram: BytesMut::with_capacity(2048),
            cartridge,
        };

        nes.cpu = CPU::create_cpu(&nes);
        return nes;
    }
}

impl Memory for Nes {
    fn read(&self, address: u16) -> u8 {}

    fn write(&self, address: u16, value: u8) {}
}