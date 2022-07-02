#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Essa função e chamada quado rolar PÂNICO
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
	loop {}
}