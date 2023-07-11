/*
 * For now it is hardcoded for MIPS
 */
 use std::mem::size_of;



 #[repr(u32)]
 enum PhdrType {
     PtNull = 0x0,
     PtLoad = 0x1,
     PtDynamic = 0x2,
     PtInterp = 0x3,
     PtNote = 0x4,
     PtPhdr = 0x6,
     PtTls = 0x7,
 }

 enum Endianness {
    Little,
    Big,
 }

 
 struct ELFDump {
     endian: Endianness,
     elf_header: Vec<u8>,
     program_header: Vec<u8>,
     padding_segment: Vec<u8>,
     segments: Vec<u8>,
 }
 
 impl ELFDump {
     fn new(endian: Endianness) -> Self 
     {
         let mut elf_dump = ELFDump {
             endian: endian,
             elf_header: Vec::new(),
             program_header: Vec::new(),
             padding_segment: Vec::new(),
             segments: Vec::new(),
         };
         elf_dump.create_header();
         elf_dump
     }
 
     fn create_header(&mut self) {
         let ehd_size = 0x34;
 
         self.elf_header = vec![0u8; ehd_size];
 
         self.elf_header[0x00..0x04].copy_from_slice(&[0x7f, 0x45, 0x4c, 0x46]); // e_ident (Magic)
         self.elf_header[0x04] = 0x01; // Class
         self.elf_header[0x05] = 0x02; // Data (endianness)
         self.elf_header[0x06] = 0x01; // Version
         self.elf_header[0x07] = 0x00; // OS/ABI
         self.elf_header[0x08] = 0x01; // ABI version
 
         self.elf_header[0x10..0x12].copy_from_slice(&0x0002.to_ne_bytes()); // e_type
         self.elf_header[0x12..0x14].copy_from_slice(&0x0008.to_ne_bytes()); // e_machine
         self.elf_header[0x14..0x18].copy_from_slice(&0x00000001.to_ne_bytes()); // e_version
         self.elf_header[0x18..0x1C].copy_from_slice(&0x0.to_ne_bytes()); // e_entry
         self.elf_header[0x1C..0x20].copy_from_slice(&0x00000034.to_ne_bytes()); // e_phoff
         self.elf_header[0x20..0x24].copy_from_slice(&0x0.to_ne_bytes()); // e_shoff
         self.elf_header[0x24..0x28].copy_from_slice(&0x70001005.to_ne_bytes()); // e_flags
         self.elf_header[0x28..0x2A].copy_from_slice(&(ehd_size as u16).to_ne_bytes()); // e_ehsize
 
         // No need to populate these bytes yet
         //self.elf_header[0x2A..0x2C].copy_from_slice(&0x0.to_ne_bytes()); // e_phentsize
         //self.elf_header[0x2C..0x2E].copy_from_slice(&0x0.to_ne_bytes()); // e_phnum
         //self.elf_header[0x2E..0x30].copy_from_slice(&0x0.to_ne_bytes()); // e_shentsize
         //self.elf_header[0x30..0x32].copy_from_slice(&0x0.to_ne_bytes()); // e_shnum
         //self.elf_header[0x32..0x34].copy_from_slice(&0x0.to_ne_bytes()); // e_shstrndx
     }
 
     fn create_program_header_table(&mut self, process: &Process) {
         let phd_entry_size = 0x20;
         let mut start_segments_offset =
             self.elf_header.len() + phd_entry_size * process.mm.mmap.len();
         let mut n = 0;
         let mut accumulation_previous_region_size = 0;
 
         let mut entry = vec![0u8; phd_entry_size];
         let mut start_addr = 0x400000;
         let length = 0x1000;
         let perms = 7;
         let p_offset = 0x0;
 
         entry[0x00..0x04].copy_from_slice(&PhdrType::PtNull.to_ne_bytes()); // p_type
         entry[0x04..0x08].copy_from_slice(&p_offset.to_ne_bytes()); // p_offset
         entry[0x08..0x0C].copy_from_slice(&start_addr.to_ne_bytes()); // p_vaddr
         entry[0x0C..0x10].copy_from_slice(&start_addr.to_ne_bytes()); // p_paddr
         entry[0x10..0x14].copy_from_slice(&length.to_ne_bytes()); // p_filez
         entry[0x14..0x18].copy_from_slice(&length.to_ne_bytes()); // p_memsz
         entry[0x18..0x1C].copy_from_slice(&perms.to_ne_bytes()); // p_flags
         entry[0x1C..0x20].copy_from_slice(&0x10000.to_ne_bytes()); // p_align
 
         n += 1;
         accumulation_previous_region_size += length;
         self.program_header.extend(entry);
 
         for region in process.mm.mmap.iter().filter(|region| {
             let filename = &region.data.name;
             filename != "[vdso]" && filename != "[vvar]"
         }) {
             let mut entry = vec![0u8; phd_entry_size];
             let start_addr = region.begin;
             let length = region.length();
             let length_mem = length;
             let perms = region.data.octal_permissions();
             let p_offset = accumulation_previous_region_size;
 
             entry[0x00..0x04].copy_from_slice(&PhdrType::PtLoad.to_ne_bytes()); // p_type
             entry[0x04..0x08].copy_from_slice(&p_offset.to_ne_bytes()); // p_offset
             entry[0x08..0x0C].copy_from_slice(&start_addr.to_ne_bytes()); // p_vaddr
             entry[0x0C..0x10].copy_from_slice(&start_addr.to_ne_bytes()); // p_paddr
             entry[0x10..0x14].copy_from_slice(&length.to_ne_bytes()); // p_filez
             entry[0x14..0x18].copy_from_slice(&length_mem.to_ne_bytes()); // p_memsz
             entry[0x18..0x1C].copy_from_slice(&perms.to_ne_bytes()); // p_flags
             entry[0x1C..0x20].copy_from_slice(&0x10000.to_ne_bytes()); // p_align
 
             n += 1;
             accumulation_previous_region_size += region.length();
             self.program_header.extend(entry);
         }
 
         self.elf_header[0x2A..0x2C].copy_from_slice(&phd_entry_size.to_ne_bytes()); // e_phentsize
         self.elf_header[0x2C..0x2E].copy_from_slice(&n.to_ne_bytes()); // e_phnum
     }
 
     fn add_padding(&mut self) {
         self.padding_segment =
             vec![b'0'; 0x1000 - self.elf_header.len() - self.program_header.len()];
     }
 
     fn binary(&self) -> Vec<u8> {
         let mut binary = Vec::new();
         binary.extend_from_slice(&self.elf_header);
         binary.extend_from_slice(&self.program_header);
         binary.extend_from_slice(&self.padding_segment);
         binary.extend_from_slice(&self.segments);
         binary
     }
 }
 