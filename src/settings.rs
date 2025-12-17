use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(Serialize, Deserialize, Hash)]
pub enum Pf2eVersion {
    Old,
    #[default]
    Remastered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Hash)]
pub struct Settings {
    pf2e_version: Pf2eVersion,
    undo_size: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self { pf2e_version: Pf2eVersion::default(), undo_size: 128 }
    }
}

impl Settings {
    pub fn get_undo_size(&self) -> usize {
        self.undo_size
    }

    pub fn set_undo_size(&mut self, value: usize) {
        self.undo_size = value
    }

    pub fn get_pf2e_version(&self) -> Pf2eVersion {
        self.pf2e_version
    }

    pub fn set_pf2e_version(&mut self, value: Pf2eVersion) {
        self.pf2e_version = value
    }
}
