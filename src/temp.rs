use std::convert::From;
use std::fs::File;
use std::io;
use std::io::Read;
use std::num;

const CPU_TEMP_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ParseIntError(num::ParseIntError)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

pub fn read() -> Result<i32, Error> {
    let mut file = File::open(CPU_TEMP_FILE)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let raw_temp = contents.trim().parse::<i32>()?;

    Ok(raw_temp / 1_000)
}

