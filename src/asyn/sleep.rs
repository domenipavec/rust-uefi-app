use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use uefi::runtime;

pub struct SleepFuture {
    end_ts: u64,
}

impl SleepFuture {
    fn get_ts() -> u64 {
        let mut result: u64 = 0;
        let t = runtime::get_time().unwrap();
        result += t.day() as u64;
        result *= 24;
        result += t.hour() as u64;
        result *= 60;
        result += t.minute() as u64;
        result *= 60;
        result += t.second() as u64;
        result *= 1_000_000_000;
        result += t.nanosecond() as u64;
        result
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<()> {
        if SleepFuture::get_ts() >= self.end_ts {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub fn sleep(t: f64) -> SleepFuture {
    SleepFuture {
        end_ts: SleepFuture::get_ts() + (t * 1_000_000_000.0) as u64,
    }
}
