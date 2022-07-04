#![no_std] // Não pode usar a biblioteca padrão mano
#![no_main] // Tira todos os runtimes iniciais do rust

use core::panic::PanicInfo;

// Essa função e chamada quado rolar PÂNICO
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
	 loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	 loop{}
}
