use std::{fs::File, io::{BufReader, Read}};

use serde::{Serialize, Deserialize};


pub enum Error {

}

pub trait Saver {
    fn save<D: Serialize, Deserialize>(&self, data: &D, dir: impl Into<String>) -> Result<(), Error>;
}

pub struct NoSaver;
impl Saver for NoSaver {
    fn save<D: Serialize, Deserialize>(&self, data: &D, dir: impl Into<String>) -> Result<(), Error> {
        Ok(())
    }
}

pub struct FileSaver;
impl Saver for FileSaver {
    fn save<D: Serialize, Deserialize>(&self, data: &D, dir: impl Into<String>) -> Result<(), Error> {                
        let file = File::open(dir.into()).unwrap();
        let mut buf = BufReader::new(file);

        let mut data = serde_json::to_string(data).unwrap();

        buf.read_to_string(&mut data).unwrap();
        Ok(())
    }
}
