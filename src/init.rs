extern crate alloc;

use log::info;

use crate::{
    asyn::{sleep, Executor, SimpleExecutor, Task},
    network::{arp, ethernet, icmp, ip},
};

pub fn init() {
    uefi::helpers::init().unwrap();
    // disable watchdog
    uefi::boot::set_watchdog_timer(0, 0xDEADBEEF, None).unwrap();

    let mut network_service = ethernet::Service::new();
    let mac_address = network_service.mac_address();
    log::info!("mac address: {:?}", mac_address);

    let ip_address = ip::Address([172, 23, 71, 108]);
    log::info!("ip address: {:?}", ip_address);

    let arp_service = arp::Service::new(ip_address, mac_address, &mut network_service);
    let mut ip_service = ip::Service::new(&mut network_service, ip_address);
    let icmp_service = icmp::Service::new(&mut ip_service);

    let executor = SimpleExecutor::new();
    network_service.start(&executor);
    arp_service.start(&executor);
    ip_service.start(&executor);
    icmp_service.start(&executor);

    executor.spawn(Task::new(hello_world(0)));
    executor.spawn(Task::new(hello_world(1)));
    executor.spawn(Task::new(hello_world(2)));

    executor.run();
}

async fn hello_world(x: u64) {
    loop {
        info!("hello world {}", x);
        sleep(1.0).await;
    }
}
