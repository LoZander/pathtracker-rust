use std::{fs, io};

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error("couldn't load savefile `{0}`, because there is no such file at that directory.")]
    LoadMissingSave(String),
    #[error("couldn't load savefile at `{0}` due to I/O error `{1}`")]
    LoadIOError(String, #[source] io::Error),
    #[error("couldn't save savefile at `{0}` due to I/O error `{1}`")]
    SaveIOError(String, #[source] io::Error),
    #[error("couldn't load savefile at `{0}` due to corruption `{1}`")]
    LoadCorruptSave(String, #[source] serde_json::Error),
    #[error("couldn't save savefile at `{0}` due to corruption of data `{1}`")]
    SaveCorruptData(String, serde_json::Error),
}

pub type Result<T> = std::result::Result<T,Error>;

pub trait Saver : Default + Clone + Sized {
    fn save<D: Serialize + DeserializeOwned>(&self, data: &D, dir: impl Into<String>) -> Result<()>;
    fn load<D: Serialize + DeserializeOwned>(&self, dir: impl Into<String>) -> Result<D>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoSaver;
impl Saver for NoSaver {
    fn save<D: Serialize + DeserializeOwned>(&self, _: &D, _: impl Into<String>) -> Result<()> {
        Ok(())
    }

    fn load<D: Serialize + DeserializeOwned>(&self, dir: impl Into<String>) -> Result<D> {
        Err(Error::LoadMissingSave(dir.into()))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FileSaver;
impl Saver for FileSaver {
    fn save<D: Serialize + DeserializeOwned>(&self, data: &D, dir: impl Into<String>) -> Result<()> {                
        let dir: String = dir.into();
        let data = serde_json::to_string_pretty(data).map_err(|err| Error::SaveCorruptData(dir.clone(), err))?;

        fs::write(dir.clone(), data).map_err(|err| Error::SaveIOError(dir, err))?;
        Ok(())
    }

    fn load<D: Serialize + DeserializeOwned>(&self, dir: impl Into<String>) -> Result<D> {
        let dir: String = dir.into();
        let json = fs::read(dir.clone()).map_err(|err| Error::LoadIOError(dir.clone(), err))?;
        let data = serde_json::from_slice(&json).map_err(|err| Error::LoadCorruptSave(dir, err))?;
        Ok(data)
    }
}
