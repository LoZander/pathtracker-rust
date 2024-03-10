use std::{fs::{self, File}, io::{BufReader, BufWriter, Read, Write}};

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone)]
pub enum Error {

}

pub trait Saver : Default + Clone + Sized {
    fn save<'de, D: Serialize + Deserialize<'de>>(&self, data: &D, dir: impl Into<String>) -> Result<(), Error>;
}

#[derive(Default)]
#[derive(Debug, Clone, Copy)]
pub struct NoSaver;
impl Saver for NoSaver {
    fn save<'de, D: Serialize + Deserialize<'de>>(&self, data: &D, dir: impl Into<String>) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Default)]
#[derive(Debug, Clone, Copy)]
pub struct FileSaver;
impl Saver for FileSaver {
    fn save<'de, D: Serialize + Deserialize<'de>>(&self, data: &D, dir: impl Into<String>) -> Result<(), Error> {                
        let data = serde_json::to_string_pretty(data).unwrap();

        fs::write("saves/auto.save", data).unwrap();
        Ok(())
    }
}
