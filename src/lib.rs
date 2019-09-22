extern crate reqwest;
extern crate serde_json;

use serde_json::Value;

pub struct Camera {
    pub addr: String,
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum Error {
    HTTP(reqwest::Error),
    StatusCode(String),
    JSON(serde_json::Error),
    Utf8(std::string::FromUtf8Error),
    MessageError,
    NotCaptured,
}

fn validate_response(res: &mut reqwest::Response) -> Result<Vec<u8>, Error> {
    if !res.status().is_success() {
        return Err(Error::StatusCode(format!("{}", res.status())));
    }

    let mut data = match res.content_length() {
        Some(len) => Vec::with_capacity(len as usize),
        None => Vec::new(),
    };

    res.copy_to(&mut data)?;
    Ok(data)
}

fn validate_json_response(res: &str) -> Result<(), Error> {
    let msg: Value = serde_json::from_str(res)?;

    if let Value::Object(msg) = msg {
        // verify msg["errCode"] == 200
        if let Some(Value::Number(num)) = msg.get("errCode") {
            if num.as_i64() != Some(200) {
                if let Some(Value::String(errmsg)) = msg.get("errMsg") {
                    return Err(Error::StatusCode(format!("{}", errmsg)));
                } else {
                    return Err(Error::StatusCode("bad errCode".to_string()));
                }
            }
        } else {
            return Err(Error::MessageError);
        }
    } else {
        return Err(Error::MessageError);
    }

    Ok(())
}

impl Camera {
    pub fn new(addr: String) -> Self {
        Camera {
            addr,
            client: reqwest::Client::new(),
        }
    }

    fn post(&self, url: String) -> Result<Vec<u8>, Error> {
        let mut res = self.client.post(&url).send()?;
        let data = validate_response(&mut res)?;

        Ok(data)
    }

    fn get(&self, url: String) -> Result<Vec<u8>, Error> {
        let mut res = self.client.get(&url).send()?;
        let data = validate_response(&mut res)?;

        Ok(data)
    }

    fn put(
        &self,
        url: String,
        data: Option<String>,
    ) -> Result<Vec<u8>, Error> {
        let mut res = match data {
            Some(body) => self.client.put(&url).body(body).send()?,
            None => self.client.put(&url).send()?,
        };

        let data = validate_response(&mut res)?;
        Ok(data)
    }

    pub fn set_iso(&self, iso: usize) -> Result<(), Error> {
        let data = self.put(
            format!("http://{}/v1/params/camera", self.addr),
            Some(format!("sv={}", iso)),
        )?;
        let json_str = String::from_utf8(data)?;
        validate_json_response(&json_str)?;

        Ok(())
    }

    pub fn get_latest_capture(&self) -> Result<Vec<u8>, Error> {
        // The image won't be out of the buffer for some time, so we may need
        // to retry multiple times.
        let mut tries = 0;
        loop {
            let res = self
                .get(format!("http://{}/v1/photos/latest/info", self.addr))?;
            let json_str = String::from_utf8(res)?;
            validate_json_response(&json_str)?;

            let latest_info: Value = serde_json::from_str(&json_str)?;
            let latest_info = if let Value::Object(obj) = latest_info {
                obj
            } else {
                return Err(Error::MessageError);
            };

            if let Some(Value::Bool(true)) = latest_info.get("captured") {
                break;
            } else {
                tries += 1;
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }

            if tries > 3 {
                return Err(Error::NotCaptured);
            }
        }

        // determine where the latest photo is
        let res =
            self.get(format!("http://{}/v1/photos/latest/info", self.addr))?;
        let json_str = String::from_utf8(res)?;
        validate_json_response(&json_str)?;

        let latest_info: Value = serde_json::from_str(&json_str)?;
        let latest_info = if let Value::Object(obj) = latest_info {
            obj
        } else {
            return Err(Error::MessageError);
        };

        let dir = if let Some(Value::String(dir)) = latest_info.get("dir") {
            dir
        } else {
            return Err(Error::MessageError);
        };

        let fname = if let Some(Value::String(file)) = latest_info.get("file")
        {
            file
        } else {
            return Err(Error::MessageError);
        };

        // download the file
        let data = self.get(format!(
            "http://{}/v1/photos/{}/{}",
            self.addr, dir, fname
        ))?;

        Ok(data)
    }

    pub fn shutter_press(&self) -> Result<(), Error> {
        let res =
            self.post(format!("http://{}/v1/camera/shoot/start", self.addr))?;
        let json_str = String::from_utf8(res)?;
        validate_json_response(&json_str)?;

        Ok(())
    }

    pub fn shutter_release(&self) -> Result<(), Error> {
        let res =
            self.post(format!("http://{}/v1/camera/shoot/finish", self.addr))?;
        let json_str = String::from_utf8(res)?;
        validate_json_response(&json_str)?;
        Ok(())
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::HTTP(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::JSON(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Error {
        Error::Utf8(e)
    }
}
