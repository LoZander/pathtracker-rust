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
    InvalidDirPath(String, #[source] io::Error),
    #[error("couldn't load savefile at `{0}` due to invalid directory path `{1}`")]
    LoadCorruptSave(String, #[source] serde_json::Error),
    #[error("couldn't save savefile at `{0}` due to serialisation error `{1}`")]
    SerialisationError(String, serde_json::Error),
}

pub type Result<T> = std::result::Result<T,Error>;

pub trait Saver : Default + Clone + Sized {
    /// Saves [`data`] to save directory [`dir`].
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - Serialisation of [`data`] fails
    /// - [`dir`] is an invalid directory.
    fn save<D: Serialize + DeserializeOwned>(&self, data: &D, dir: impl Into<String>) -> Result<()>;

    /// Loads data of type [`D`] from directory [`dir`].
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - Loading raw data from the file fails
    /// - Deserialisation of data into type [`D`] fails.
    fn load<D: Serialize + DeserializeOwned>(&self, dir: impl Into<String>) -> Result<D>;
}

#[allow(clippy::module_name_repetitions)]
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

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FileSaver;
impl Saver for FileSaver {
    fn save<D: Serialize + DeserializeOwned>(&self, data: &D, dir: impl Into<String>) -> Result<()> {                
        let dir: String = dir.into();
        let data = serde_json::to_string_pretty(data).map_err(|err| Error::SerialisationError(dir.clone(), err))?;

        fs::write(dir.clone(), data).map_err(|err| Error::InvalidDirPath(dir, err))?;
        Ok(())
    }

    fn load<D: Serialize + DeserializeOwned>(&self, dir: impl Into<String>) -> Result<D> {
        let dir: String = dir.into();
        let json = fs::read(dir.clone()).map_err(|err| Error::LoadIOError(dir.clone(), err))?;
        let data = serde_json::from_slice(&json).map_err(|err| Error::LoadCorruptSave(dir, err))?;
        Ok(data)
    }
}
