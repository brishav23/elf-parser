use std::{
	alloc::{alloc, Layout, dealloc},
	fs::File,
	io::{Seek, SeekFrom, Read},
	mem,
	slice,
	vec,
};

use crate::types::*;

// sh_type
const SHT_NULL: Elf64_Word = 0;
const SHT_PROGBITS: Elf64_Word = 1;
const SHT_SYMTAB: Elf64_Word = 2;
const SHT_STRTAB: Elf64_Word = 3;
const SHT_RELA: Elf64_Word = 4;
const SHT_HASH: Elf64_Word = 5;
const SHT_DYNAMIC: Elf64_Word = 6;
const SHT_NOTE: Elf64_Word = 7;
const SHT_NOBITS: Elf64_Word = 8; // .bss
const SHT_REL: Elf64_Word = 9;
const SHT_SHLIB: Elf64_Word = 10;
const SHT_DYNSYM: Elf64_Word = 11;
const SHT_LOOS: Elf64_Word = 0x60000000; // Environment-specific use
const SHT_HIOS: Elf64_Word = 0x6FFFFFFF;
const SHT_LOPROC: Elf64_Word = 0x70000000; // Processor-specific use
const SHT_HIPROC: Elf64_Word = 0x7FFFFFFF;

// sh_flags
const SHF_WRITE: Elf64_Xword = 0x1; // Section contains writable data
const SHF_ALLOC: Elf64_Xword = 0x2; // Section is allocated in memory 
const SHF_EXECINSTR: Elf64_Xword = 0x4; // Section contains executable 
const SHF_MASKOS: Elf64_Xword = 0x0F000000; // Environment-specific use
const SHF_MASKPROC: Elf64_Xword = 0xF0000000; // Processor-specific use

#[repr(C)]
pub struct Elf64_Shdr {
	pub sh_name: Elf64_Word,		// Section name
	pub sh_type: Elf64_Word,		// Section type
	pub sh_flags: Elf64_Xword,		// Section attributes
	pub sh_addr: Elf64_Addr,		// Virtual address in memory
	pub sh_offset: Elf64_Off,		// Offset in file
	pub sh_size: Elf64_Xword,		// Size of section
	pub sh_link: Elf64_Word,		// Link to other section
	pub sh_info: Elf64_Word,		// Miscellaneous information
	pub sh_addralign: Elf64_Xword,	// Address alignment boundary
	pub sh_entsize: Elf64_Xword,	// Size of entries, if section has table
}

pub struct ShdrTab {
	pub tab: Vec<Elf64_Shdr>,
}

impl ShdrTab {
	pub fn read_shdr_table(f: &mut File, off: u64, n: usize) -> Self {
		// Create table on heap
		let mut tab: Vec<Elf64_Shdr> = Vec::with_capacity(n);

		// Get buffer to read into
		let p = tab.as_mut_ptr() as *mut u8;
		let sz = n * mem::size_of::<Elf64_Shdr>();
		let buf = unsafe {
			slice::from_raw_parts_mut(p, sz)
		};

		// Seek to offset and read
		f.seek(SeekFrom::Start(off as u64)).unwrap();
		f.read_exact(buf).unwrap();

		// Since this uses unsafe rust to read contiguous repr(C) structs into the underlying buffer,
		// we need to manually set the length of the vector to the number of structs
		// we read into the buffer.
		unsafe {
			tab.set_len(n);
		}

		// Return table
		Self { tab: tab }
	}

	pub fn print_types(&self) {
		println!("Sections:");
		for i in &self.tab[..] {
			match i.sh_type {
				0 => {
					println!("SHT_NULL");
				},
				1 => {
					println!("SHT_PROGBITS");
				},
				2 => {
					println!("SHT_SYMTAB");
				},
				3 => {
					println!("SHT_STRTAB");
				},
				4 => {
					println!("SHT_RELA");
				},
				5 => {
					println!("SHT_HASH");
				},
				6 => {
					println!("SHT_DYNAMIC");
				},
				7 => {
					println!("SHT_NOTE");
				},
				8 => {
					println!("SHT_NOBITS");
				},
				9 => {
					println!("SHT_REL");
				},
				10 => {
					println!("SHT_SHLIB");
				},
				11 => {
					println!("SHT_DYNSYM");
				},
				0x60000000 => {
					println!("SHT_LOOS");
				},
				0x6FFFFFFF => {
					println!("SHT_HIOS");
				},
				0x70000000 => {
					println!("SHT_LOPROC");
				},
				0x7FFFFFFF => {
					println!("SHT_HIPROC");
				},
				_ => {
					println!("OTHER");
				},
			}
		}
	}
}

impl IntoIterator for ShdrTab {
	type Item = Elf64_Shdr;
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.tab.into_iter()
	}
}

// pub struct Elf64_Shdr_Wrapper {
// 	pub ptr: *mut Elf64_Shdr,
// }


// impl Elf64_Shdr_Wrapper {
// 	pub fn new() -> Self {
// 		let p = unsafe {
// 			alloc(Layout::new::<Elf64_Shdr_Wrapper>())
// 		};
// 		Elf64_Shdr_Wrapper { ptr: p as *mut Elf64_Shdr }
// 	}

// 	// Allocate space for section header table on heap and return
// 	pub fn read_shdr_table(f: &mut File, off: usize) {
// 		let p = alloc(Layout::size(&self))
// 	}
// }

// impl Drop for Elf64_Shdr_Wrapper {
// 	fn drop(&mut self) {
// 		let p = self.ptr as *mut u8;
// 		unsafe {
// 			dealloc(p, Layout::new::<Elf64_Shdr>());
// 		}
// 	}
// }