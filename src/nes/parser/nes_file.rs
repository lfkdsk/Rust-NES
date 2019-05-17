#[allow(dead_code)]
#[warn(unused_imports)]
use std::{
    fs::File,
    io::Result,
};
use std::fs;
use std::io::{ErrorKind, Read};
use std::io::Error;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use bytes::BytesMut;

const NES_FILE_MAGIC: u32 = 0x1a53454e;

#[derive(Debug)]
pub struct NesFileHeader {
    magic_code: u32,
    // NES
    num_prg: u8,
    // number of PRG-ROM banks (16KB each)
    num_chr: u8,
    // number of CHR-ROM banks (8KB each)
    control1: u8,
    // control bits
    control2: u8,
    // control bits
    num_ram: u8,
    // PRG-RAM size (x 8KB)
    padding: [u8; 8], // Not used at this tume but MUST BE ALL ZEROS or games will not work
}

pub struct Cartridge {
    pub prg: BytesMut,
    pub chr: BytesMut,
    pub mapper: u8,
    pub mirror: u8,
    pub battery: u8,
}

pub fn read_nes_rom(path: &str) -> Result<Cartridge> {
    let mut rom = File::open(path).expect("cannot read nes file.");
    let magic_code: u32 = rom.read_u32::<LittleEndian>().unwrap();
    let num_prg: u8 = rom.read_u8().unwrap();
    let num_chr: u8 = rom.read_u8().unwrap();
    let control1: u8 = rom.read_u8().unwrap();
    let control2: u8 = rom.read_u8().unwrap();
    let num_ram: u8 = rom.read_u8().unwrap();
    let mut padding = [0; 8];
    rom.read(&mut padding);

    if magic_code != NES_FILE_MAGIC {
        return Err(Error::new(ErrorKind::InvalidInput, "invalid .nes file."));
    }

    let header = NesFileHeader {
        magic_code,
        num_prg,
        num_chr,
        control1,
        control2,
        num_ram,
        padding,
    };

    // mapper type
    let mapper1 = header.control1 >> 4;
    let mapper2 = header.control1 >> 4;
    let mapper = mapper1 | mapper2 << 4;

    // mirroring type
    let mirror1 = header.control1 & 1;
    let mirror2 = (header.control1 >> 3) & 1;
    let mirror = mirror1 | mirror2 << 1;

    // battery-backed RAM
    let battery = (header.control1 >> 1) & 1;


    // read trainer if present (unused)
    if header.control1 & 4 == 4 {
        let mut train = [0; 512];
        rom.read(&mut train);
    }

    let mut prg = BytesMut::with_capacity((header.num_prg as i32 * 16384) as usize);
    rom.read(&mut prg);

    let mut chr = BytesMut::with_capacity((header.num_chr as i32 * 8192) as usize);
    rom.read(&mut chr);

    Ok(Cartridge {
        prg,
        chr,
        mapper,
        mirror,
        battery,
    })
}


