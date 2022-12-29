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

#[repr(align(2))]
struct Elf64_Shdr {
	sh_name: Elf64_Word,		// Section name
	sh_type: Elf64_Word,		// Section type
	sh_flags: Elf64_Xword,		// Section attributes
	sh_addr: Elf64_Addr,		// Virtual address in memory
	sh_offset: Elf64_Off,		// Offset in file
	sh_size: Elf64_Xword,		// Size of section
	sh_link: Elf64_Word,		// Link to other section
	sh_info: Elf64_Word,		// Miscellaneous information
	sh_addralign: Elf64_Xword,	// Address alignment boundary
	sh_entsize: Elf64_Xword,	// Size of entries, if section has table
}