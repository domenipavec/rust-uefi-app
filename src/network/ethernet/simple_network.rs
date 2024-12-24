extern crate alloc;

use alloc::boxed::Box;
use core::{
    error::Error,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use uefi::{boot, proto::network::snp, Status};

use super::MacAddress;

pub struct SimpleNetwork {
    sn: boot::ScopedProtocol<snp::SimpleNetwork>,
}

impl SimpleNetwork {
    pub fn new() -> SimpleNetwork {
        let handle = boot::get_handle_for_protocol::<snp::SimpleNetwork>().unwrap();
        let sn = boot::open_protocol_exclusive::<snp::SimpleNetwork>(handle).unwrap();

        sn.shutdown().unwrap();
        sn.stop().unwrap();
        sn.start().unwrap();
        sn.initialize(0, 0).unwrap();
        sn.get_interrupt_status().unwrap();
        sn.reset_statistics().unwrap();

        SimpleNetwork { sn }
    }

    pub fn receive<'a>(&'a self, buf: &'a mut [u8]) -> ReceiveFuture<'a> {
        ReceiveFuture {
            sn: &self.sn,
            buffer: buf,
        }
    }

    // TODO: improve speed
    pub fn transmit<'a>(&'a self, buf: &'a [u8]) -> Result<(), Box<dyn Error>> {
        self.sn.transmit(0, buf, None, None, None)?;
        while self.sn.get_recycled_transmit_buffer_status()?.is_none() {}
        Ok(())
    }

    pub fn mac_address(&self) -> MacAddress {
        MacAddress(self.sn.mode().current_address.0[0..6].try_into().unwrap())
    }
}

pub struct ReceiveFuture<'a> {
    buffer: &'a mut [u8],
    sn: &'a boot::ScopedProtocol<snp::SimpleNetwork>,
}

impl Future for ReceiveFuture<'_> {
    type Output = Result<usize, Box<dyn Error>>;

    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        match self.sn.receive(self.buffer, None, None, None, None) {
            Ok(v) => Poll::Ready(Ok(v)),
            Err(e) => {
                if e.status() != Status::NOT_READY {
                    Poll::Ready(Err(Box::new(e)))
                } else {
                    Poll::Pending
                }
            }
        }
    }
}
