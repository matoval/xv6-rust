#![no_std]
#![no_main]

mod mem;
use mem::stosb;

const SECTSIZE: usize = 512;

type Uchar = u8;

const ELF_MAGIC: u32 = 0x464c457f; // 0x7F 'E' 'L' 'F'

#[repr(C)]
struct ElfHdr {
    magic: u32,
    _other: [u8; 12], // skip irrelevant fields
    phoff: u32,
    _other2: [u8; 8], 
    phnum: u16,
    _other3: [u8; 6], 
    entry: u32,
}

#[repr(C)]
struct ProgHdr {
    _type: u32,
    off: u32,
    vaddr: u32,
    paddr: u32,
    filesz: u32,
    memsz: u32,
    flags: u32,
    align: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn bootmain() -> ! {
    let elf: *mut ElfHdr = 0x10000 as *mut ElfHdr; // scratch space

    unsafe {
        // Read first 4096 bytes of disk into elf
        readseg(elf as *mut Uchar, 4096, 0);

        // Check ELF magic number
        if (*elf).magic != ELF_MAGIC {
            loop {} // halt, or trigger error handling here
        }

        // Calculate program header range
        let ph = (elf as *const u8).add((*elf).phoff as usize) as *const ProgHdr;
        let eph = ph.add((*elf).phnum as usize);

        let mut cur_ph = ph;
        while cur_ph < eph {
            let pa = (*cur_ph).paddr as *mut Uchar;

            // Read segment into physical address
            readseg(pa, (*cur_ph).filesz, (*cur_ph).off);

            // Zero out bss region if memsz > filesz
            if (*cur_ph).memsz > (*cur_ph).filesz {
                stosb(pa.add((*cur_ph).filesz as usize), 0, ((*cur_ph).memsz - (*cur_ph).filesz) as usize);
            }
            cur_ph = cur_ph.add(1);
        }

        // Call entry point, never returns
        let entry: extern "C" fn() = core::mem::transmute((*elf).entry as usize);
        entry();

        // If entry returns, halt
        loop {}
    }
}


unsafe fn inb(port: u16) -> u8 {
  let data: u8;
  unsafe {
    core::arch::asm!("in al, dx", out("al") data, in("dx") port);
  }
  data
}

unsafe fn outb(port: u16, val: u8) {
  unsafe {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val);
  }
}

unsafe fn insl(port: u16, dst: *mut u8, count: usize) {
  let dst = dst as *mut u32;
  unsafe {
    core::arch::asm!(
      "cld; rep insw",
      in("dx") port,
      inout("edi") dst => _,
      inout("ecx") count => _,
      options(nostack, preserves_flags)
    );
  }
}

unsafe fn waitdisk() {
  unsafe {
    // Wait for disk ready
    while(inb(0x1F7) & 0xC0) != 0x40 {}
  }
}

unsafe fn readsect(dst: *mut u8, offset: u32) {
  unsafe {
    waitdisk();
  
    outb(0x1F2, 1); // Sector count = 1
    outb(0x1F3, (offset & 0xFF) as u8);
    outb(0x1F4, ((offset >> 8) & 0xFF) as u8);
    outb(0x1F5, ((offset >> 16) & 0xFF) as u8);
    outb(0x1F6, (((offset >> 24) & 0x0F) | 0xE0) as u8); // LBA, drive 0
    outb(0x1F7, 0x20); // Command: read sectors
  
    waitdisk();
  
    insl(0x1F0, dst, SECTSIZE / 4); // Read 512 bytes = 128 dwords
  }
}

unsafe fn readseg(mut pa: *mut u8, count: u32, mut offset: u32) {
  unsafe {
    let epa = pa.add(count as usize);

    // Round pa down to the nearest sector boundary
    let misalignment = (offset as usize) % SECTSIZE;
    pa = pa.sub(misalignment);
  
    // Convert byte offset to sector number (kernel starts at sector 1)
    offset = (offset / SECTSIZE as u32) + 1;
  
    // Read sectors until we've covered the target range
    while pa < epa {
      readsect(pa, offset);
      pa = pa.add(SECTSIZE);
      offset += 1;
    }
  }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}