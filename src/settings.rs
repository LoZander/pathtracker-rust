use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(Serialize, Deserialize, Hash)]
pub enum Pf2eVersion {
    Old,
    #[default]
    Remastered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(Serialize, Deserialize, Hash)]
pub struct Settings {
    pub pf2e_version: Pf2eVersion
}
