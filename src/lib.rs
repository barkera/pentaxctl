extern crate reqwest;

use std::net::SocketAddr;

pub struct Camera {
    pub addr: SocketAddr,
}

pub enum Error {
}

impl Camera {
    pub fn new(addr: SocketAddr) -> Self {
        Camera {
            addr,
        }
    }

    pub fn set_iso(&self, iso: usize) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn get_last_capture(&self) -> Result<Vec<u8>, Error> {
        unimplemented!();
    }

    pub fn start_bulb_capture(&self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn stop_bulb_capture(&self) -> Result<(), Error> {
        unimplemented!();
    }
}
