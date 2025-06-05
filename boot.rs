#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn bootmain() -> ! {
  const SECTSIZE: usize = 512;
  loop{}
}