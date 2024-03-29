#![no_std]
#![no_main]

mod hello;
mod hello_map;
mod hello_buffer;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
