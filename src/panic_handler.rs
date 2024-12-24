use log::info;
use uefi::boot;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    info!("[PANIC]: {}", info);

    // Give the user some time to read the message
    loop {
        boot::stall(10_000_000);
    }
}
