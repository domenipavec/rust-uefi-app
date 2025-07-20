extern crate alloc;

use alloc::sync::Arc;
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

    let arp_service = Arc::new(arp::Service::new(
        ip_address,
        mac_address,
        &mut network_service,
    ));
    let mut ip_service = ip::Service::new(&mut network_service, arp_service.clone(), ip_address);
    let mut icmp_service = icmp::Service::new(&mut ip_service);

    let executor = SimpleExecutor::new();

    executor.spawn(Task::new(hello_world(0)));
    executor.spawn(Task::new(hello_world(1)));
    executor.spawn(Task::new(hello_world(2)));

    let pinger1 = icmp_service.open(ip::Address([172, 23, 71, 14]));
    executor.spawn(Task::new(ping(pinger1)));

    network_service.start(&executor);
    arp_service.start(&executor);
    ip_service.start(&executor);
    icmp_service.start(&executor);

    executor.run();
}

async fn hello_world(x: u64) {
    loop {
        info!("hello world {}", x);
        sleep(60.0).await;
    }
}

async fn ping(pinger: icmp::Socket) {
    loop {
        pinger.send(&[1, 2, 3]).await;
        match pinger.receive(1.0).await {
            Some(response) => {
                info!("response from {:?}", response.ip.source_address());
                sleep(1.0).await
            }
            None => info!("timeout waiting for pinger response"),
        };
    }
}
