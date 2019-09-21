extern crate reqwest;

pub struct Camera {
    pub addr: String,
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum Error {
    HTTP(reqwest::Error)
}

impl Camera {
    pub fn new(addr: String) -> Self {
        Camera {
            addr,
            client: reqwest::Client::new(),
        }
    }

    fn post(&self, url: String) -> Result<(), Error> {
        self.client.post(&url).send()?;
        Ok(())
    }

    pub fn set_iso(&self, iso: usize) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn get_lastest_capture(&self) -> Result<Vec<u8>, Error> {
        unimplemented!();
    }

    pub fn shutter_press(&self) -> Result<(), Error> {
        self.post(format!("http://{}/v1/camera/shoot/start", self.addr))
    }

    pub fn shutter_release(&self) -> Result<(), Error> {
        self.post(format!("http://{}/v1/camera/shoot/finish", self.addr))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::HTTP(e)
    }
}
