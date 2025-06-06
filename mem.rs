#[unsafe(no_mangle)]
pub extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *s.add(i) = c as u8;
        }
    }
    s
}

#[unsafe(no_mangle)]
pub extern "C" fn stosb(addr: *mut u8, data: u8, count: usize) {
    unsafe {
        for i in 0..count {
            *addr.add(i) = data;
        }
    }
}

