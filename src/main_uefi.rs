use uefi::prelude::*;

#[entry]
fn uefi_entry() -> Status {
    crate::init::init();
    Status::SUCCESS
}
