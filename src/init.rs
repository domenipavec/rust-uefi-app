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

    let executor = Arc::new(SimpleExecutor::new());
    executor.spawn(Task::new(init_async(executor.clone())));

    executor.run();
}

async fn init_async(executor: Arc<dyn Executor>) {
    let mut network_service = ethernet::Service::new();
    let mac_address = network_service.mac_address();
    log::info!("mac address: {:?}", mac_address);

    let ip_address = ip::Address([172, 23, 71, 108]);
    log::info!("ip address: {:?}", ip_address);
    let netmask = ip::Address([255, 255, 255, 0]);
    log::info!("netmask: {:?}", netmask);
    let gateway = ip::Address([172, 23, 71, 1]);
    log::info!("gateway: {:?}", gateway);

    let arp_service = Arc::new(arp::Service::new(
        ip_address,
        mac_address,
        &mut network_service,
    ));
    let ip_service = Arc::new(ip::Service::new(
        &mut network_service,
        arp_service.clone(),
        ip_address,
        netmask,
        gateway,
    ));
    let mut icmp_service = icmp::Service::new(ip_service.clone()).await;

    executor.spawn(Task::new(hello_world(0)));
    executor.spawn(Task::new(hello_world(1)));
    executor.spawn(Task::new(hello_world(2)));

    let pinger1 = icmp_service.open(ip::Address([172, 23, 71, 14]));
    executor.spawn(Task::new(ping(pinger1)));

    let pinger2 = icmp_service.open(ip::Address([8, 8, 8, 8]));
    executor.spawn(Task::new(ping(pinger2)));

    // let pinger3 = icmp_service.open(ip::Address([172, 23, 71, 213]));
    // executor.spawn(Task::new(ping(pinger3)));

    network_service.start(executor.clone());
    arp_service.start(executor.clone());
    ip_service.start(executor.clone());
    icmp_service.start(executor.clone());
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
            None => info!(
                "timeout waiting for pinger response from {:?}",
                pinger.ip_address(),
            ),
        };
    }
}
