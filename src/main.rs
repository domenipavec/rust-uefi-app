#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

// mod arp;
mod asyn;
// mod icmp;
mod init;
// mod ip;
mod network;

#[cfg(not(test))]
mod main_uefi;

#[cfg(not(test))]
mod allocator;

#[cfg(not(test))]
mod panic_handler;

#[cfg(test)]
fn main() {
    init::init();
}
