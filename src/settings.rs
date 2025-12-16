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
    pub pf2e_version: Pf2eVersion,
    pub undo_size: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self { pf2e_version: Pf2eVersion::default(), undo_size: 128 }
    }
}
