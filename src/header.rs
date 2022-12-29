use crate::types::*;
use std::alloc::{Layout, alloc, dealloc};
use std::fs::{File};
use std::io::{Seek, SeekFrom, Read};
use std::{slice, mem};

// Used as index into e_ident in header
const EI_MAG0: u8 = 0;
const EI_MAG1: u8 = 1;
const EI_MAG2: u8 = 2;
const EI_MAG3: u8 = 3;
const EI_CLASS: u8 = 4;
const EI_DATA: u8 = 5;
const EI_VERSION: u8 = 6;
const EI_OSABI: u8 = 7;
const EI_ABIVERSION: u8 = 8;
const EI_PAD: u8 = 9;
const EI_NIDENT: u8 = 16;

// e_ident[EI_CLASS]
const ELFCLASS32: u8 = 1; // 32-bit objects
const ELFCLASS64: u8 = 2; // 64-bit objects

// e_ident[EI_DATA]
const ELFDATA2LSB: u8 = 1; // little-endian
const ELFDATA2MSB: u8 = 2; // big-endian

// e_ident[EI_OSABI]
const ELFOSABI_SYSV: u8 = 0; // System V ABI
const ELFOSABI_HPUX: u8 = 1; // HP-UX operating system
const ELFOSABI_STANDALONE: u8 = 255; // Standalone (embedded) application

// e_type
const ET_NONE: u16 = 0; // No file type
const ET_REL: u16 = 1; // Relocatable object file
const ET_EXEC: u16 = 2; // Executable file
const ET_DYN: u16 = 3; // Shared object file
const ET_CORE: u16 = 4; // Core file
const ET_LOOS: u16 = 0xFE00; // Environment-specific use
const ET_HIOS: u16 = 0xFEFF;
const ET_LOPROC: u16 = 0xFF00; // Processor-specific use
const ET_HIPROC: u16 = 0xFFFF;

//#[repr(align(2))]
#[repr(C)]
pub struct Elf64_Ehdr {
	pub e_ident: [u8; 16],
	pub e_type: Elf64_Half,
	pub e_machine: Elf64_Half,
	pub e_version: Elf64_Word,
	pub e_entry: Elf64_Addr,
	pub e_phoff: Elf64_Off,
	pub e_shoff: Elf64_Off,
	pub e_flags: Elf64_Word,
	pub e_ehsize: Elf64_Half,
	pub e_phentsize: Elf64_Half,
	pub e_phnum: Elf64_Half,
	pub e_shentsize: Elf64_Half,
	pub e_shnum: Elf64_Half,
	pub e_shstrndx: Elf64_Half,
}

impl Elf64_Ehdr {
	pub fn new() -> Box<Elf64_Ehdr> {
		let p = unsafe {
			alloc(Layout::new::<Elf64_Ehdr>()) as *mut Elf64_Ehdr
		};
		let b = unsafe {
			Box::from_raw(p)
		};
		b
	}

	pub fn read_ehdr(f: &mut File) -> Box<Elf64_Ehdr> {
		let p = unsafe {
			alloc(Layout::new::<Elf64_Ehdr>())
		};
		let slc = unsafe {
			slice::from_raw_parts_mut(p, mem::size_of::<Elf64_Ehdr>())
		};

		f.seek(SeekFrom::Start(0)).unwrap();
		f.read_exact(slc).unwrap();

		let b = unsafe {
			Box::from_raw(p as *mut Elf64_Ehdr)
		};
		b
	}

	pub fn print_magic(&self) {
		print!("Magic:\t");
		for i in self.e_ident {
			print!("{:#02x} ", i);
		}
		println!();
	}

	pub fn print_class(&self) {
		print!("Class:\t");
		match self.e_ident[EI_CLASS as usize] {
			ELFCLASS32 => {
				print!("ELF32");
			},
			ELFCLASS64 => {
				print!("ELF64");
			},
			_ => {
				print!("Problem with ELF header");
			},
		};
		println!();
	}

	pub fn print_data(&self) {
		print!("Data:\t");
		match self.e_ident[EI_DATA as usize] {
			ELFDATA2LSB => {
				print!("Little endian");
			},
			ELFDATA2MSB => {
				print!("Big endian");
			},
			_ => {
				print!("Problem with ELF header");
			},
		};
		println!();
	}

	pub fn print_os_abi(&self) {
		print!("OS/ABI:\t");
		match self.e_ident[EI_OSABI as usize] {
			ELFOSABI_SYSV => {
				print!("System V");
			},
			ELFOSABI_HPUX => {
				print!("HP-UX");
			},
			ELFOSABI_STANDALONE => {
				print!("Standalone/embedded");
			},
			_ => {
				print!("Problem with ELF header");
			},
		};
		println!();
	}

	pub fn print_type(&self) {
		print!("Type:\t");
		match self.e_type {
			ET_NONE => {
				print!("NONE");
			},
			ET_REL => {
				print!("REL");
			},
			ET_EXEC => {
				print!("EXEC");
			},
			ET_DYN => {
				print!("DYN");
			},
			ET_CORE => {
				print!("CORE");
			},
			ET_LOOS => {
				print!("LOOS");
			},
			ET_HIOS => {
				print!("HIOS");
			},
			ET_LOPROC => {
				print!("LOPROC");
			},
			ET_HIPROC => {
				print!("HIPROC");
			},
			_ => {
				print!("Problem with ELF header");
			},
		};
		println!();
	}
}
