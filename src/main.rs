use std::{env, fs::File, io, mem::{self, MaybeUninit}, slice, alloc};
use io::{Read};
use alloc::{alloc, Layout};

pub mod types {
	pub type Elf64_Addr = u64;
	pub type Elf64_Off = u64;
	pub type Elf64_Half = u16;
	pub type Elf64_Word = u32;
	pub type Elf64_Sword = i32;
	pub type Elf64_Xword = u64;
	pub type Elf64_Sxword = i64;
}

mod header;
use header::{Elf64_Ehdr};
mod section;
mod symbol_table;

fn main() {
	let argv: Vec<String> = env::args().collect();
	// use assertion for now, properly handle later
	assert!(argv.len() == 2, "Provide elf file as argument");
	println!("Filename: {}", argv[1]);

	let mut f: File = File::options().read(true).create(false).open(&argv[1]).unwrap();
	let header = Elf64_Ehdr::read_ehdr(&mut f);

	header.print_magic();
	header.print_class();
	header.print_data();
	header.print_os_abi();
	header.print_type();

	// for i in header.e_ident {
	// 	print!("{:#02x} ", i);
	// }
	// println!();
	// let p: *mut Elf64_Ehdr = (&mut hdr_slice).as_mut_ptr() as *mut Elf64_Ehdr;
}
