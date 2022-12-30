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
use header::{Elf64_Ehdr_Wrapper};

mod section;
use section::Elf64_Shdr;

use crate::section::{ShdrTab};

mod symbol_table;

fn main() {
	let argv: Vec<String> = env::args().collect();
	// use assertion for now, properly handle later
	assert!(argv.len() == 2, "Provide elf file as argument");
	println!("Filename: {}", argv[1]);

	let mut f: File = File::options().read(true).create(false).open(&argv[1]).unwrap();
	let header = Elf64_Ehdr_Wrapper::read_ehdr(&mut f);

	header.print_magic();
	header.print_class();
	header.print_data();
	header.print_os_abi();
	header.print_type();

	let sht_off = header.sht_off() as u64;
	let sh_num = header.sh_num() as usize;

	let sht = ShdrTab::read_shdr_table(&mut f, sht_off, sh_num);
	sht.print_types();
}
