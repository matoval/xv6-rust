#![no_std]
#![no_main]
#![feature(asm)]

use core::arch::asm;

// These constants come from xv6 headers (adjust as needed)
const CR0_PG: u32 = 1 << 31;
const CR0_WP: u32 = 1 << 16;
const CR4_PSE: u32 = 1 << 4;
const KSTACKSIZE: usize = 4096;

extern "C" {
    static entrypgdir: u32;
    static main: fn() -> !;
}

// Align stack as a global symbol
#[no_mangle]
#[link_section = ".bss.stack"]
static mut STACK: [u8; KSTACKSIZE] = [0; KSTACKSIZE];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "mov eax, cr4",
            "or eax, {cr4_pse}",
            "mov cr4, eax",

            "mov eax, {pgdir}",
            "mov cr3, eax",

            "mov eax, cr0",
            "or eax, {cr0_flags}",
            "mov cr0, eax",

            "mov esp, {stack_top}",

            "jmp {main_fn}",

            cr4_pse = const CR4_PSE,
            cr0_flags = const (CR0_PG | CR0_WP),
            pgdir = sym entrypgdir,
            stack_top = const STACK_TOP,
            main_fn = sym main,

            out("eax") _,
            options(noreturn)
        );
    }
    loop {} // never reached
}

#[no_mangle]
pub static STACK_TOP: usize = unsafe { &STACK as *const _ as usize + KSTACKSIZE };