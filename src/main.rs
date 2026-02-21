#![no_std]
#![no_main]

use core::panic::PanicInfo;

fn main() {
    real_main();
}

fn real_main() -> ! {
    loop {
        // ...
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
