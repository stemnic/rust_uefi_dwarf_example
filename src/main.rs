#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "C" fn main(_h: *mut core::ffi::c_void, _st: *mut core::ffi::c_void) -> usize {
    let _test1: u32 = 0xdeadbeef;
    let _test2: u64 = 0xdeadbeef0badf00d;
    let _test3: u16 = 0xcafe;
    0
}
