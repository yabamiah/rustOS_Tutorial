#![no_std] // Não pode usar a biblioteca padrão mano
#![no_main] // Tira todos os runtimes iniciais do rust

use core::panic::PanicInfo;

// Essa função e chamada quado rolar PÂNICO
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Bem-Vindo meu Anjo!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

mod vga_buffer;
