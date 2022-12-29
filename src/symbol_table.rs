use crate::types::*;

// Symbol bindings (high order bits, so shift left)
const STB_LOCAL: u8 = 0u8.overflowing_shl(4).0; // Not visible outside the object file 
const STB_GLOBAL: u8 = 1u8.overflowing_shl(4).0; // Global symbol, visible to all object files
const STB_WEAK: u8 = 2u8.overflowing_shl(4).0; // Global scope, but with lower precedence than global symbols
const STB_LOOS: u8 = 10u8.overflowing_shl(4).0; // Environment-specific use
const STB_HIOS: u8 = 12u8.overflowing_shl(4).0;
const STB_LOPROC: u8 = 13u8.overflowing_shl(4).0; // Processor-specific use
const STB_HIPROC: u8 = 15u8.overflowing_shl(4).0;

// Symbol types (low order bits)
const STT_NOTYPE: u8 = 0; // No type specified (e.g., an absolute symbol)
const STT_OBJECT: u8 = 1; // Data object
const STT_FUNC: u8 = 2; // Function entry point
const STT_SECTION: u8 = 3; // Symbol is associated with a section
const STT_FILE: u8 = 4; // Source file associated with the object file
const STT_LOOS: u8 = 10; // Environment-specific use
const STT_HIOS: u8 = 12;
const STT_LOPROC: u8 = 13; // Processor-specific use
const STT_HIPROC: u8 = 15;


struct Elf64_Sym {
	st_name: Elf64_Word,	/* Symbol name */
	st_info: u8, 			/* Type and Binding attributes */
	st_other: u8,			/* Reserved */
	st_shndx: Elf64_Half,	/* Section table index */
	st_value: Elf64_Addr,	/* Symbol value */
	st_size: Elf64_Xword,	/* Size of object (e.g., common) */
}