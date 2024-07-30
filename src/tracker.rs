use std::{cmp::Ordering, collections::HashSet};
use serde::{Deserialize, Serialize};

use thiserror::Error;

use crate::{character::{Chr, Health}, conditions::{condition_manager::ConditionManager, Condition}, saver::{self, Saver}};

#[derive(Debug, Error)]
pub enum Error {
    #[error("cannot add character with name `{0}` as there is already a character with this name.")]
    AddDupError(String),

    #[error("cannot remove a character of name `{0}` as no such character exists.")]
    RmNoneError(String),

    #[error("cannot modify character `{0}` as no such character exists.")]
    ChangeNoneError(String),

    #[error("cannot rename `{old}` into `{new}` as there is already a character with this name.")]
    RenameDupError { old: String, new: String },

    #[error("load error: `{0}`")]
    LoadError(#[from] saver::Error)
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::AddDupError(x), Error::AddDupError(y)) |
            (Error::RmNoneError(x), Error::RmNoneError(y)) |
            (Error::ChangeNoneError(x), Error::ChangeNoneError(y)) => x == y,
            (Error::RenameDupError { old: old1, new: new1 }, 
                Error::RenameDupError { old: old2, new: new2 }) => 
                    old1 == old2 && new1 == new2,
            (Error::LoadError(_), Error::LoadError(_)) => true,
            _ => false
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Tracker<S: Saver> {
    chrs: Vec<Chr>,
    in_turn_index: Option<usize>,
    saver: S,
    cm: ConditionManager
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
struct TrackerData {
    chrs: Vec<Chr>,
    in_turn_index: Option<usize>,
    cm: ConditionManager
}

impl<S: Saver> From<Tracker<S>> for TrackerData {
    fn from(value: Tracker<S>) -> Self {
        TrackerData {
            chrs: value.chrs,
            in_turn_index: value.in_turn_index,
            cm: value.cm
        }
    }
}

impl<S: Saver> From<TrackerData> for Tracker<S> {
    fn from(value: TrackerData) -> Self {
        Tracker {
            chrs: value.chrs,
            in_turn_index: value.in_turn_index,
            saver: S::default(),
            cm: value.cm
        }
    }
}

impl<S: Saver> Default for Tracker<S> {
    fn default() -> Self {
        Builder::default().build()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MovedStatus {
    Skipped(Chr),
    TwoTurns(Chr),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Builder<S: Saver> {
    chrs: Vec<Chr>,
    in_turn_index: Option<usize>,
    saver: S,
    cm: ConditionManager
}

impl<S: Saver> Builder<S> {
    /// Creates a new [`TrackerBuilder<S>`].
    #[must_use]
    pub fn new(saver: S) -> Self {
        Self { chrs: vec![], in_turn_index: None, saver, cm: ConditionManager::new() }
    }

    /// Adds a [`saver`] [`S`] to the [`TrackerBuilder<S>`].
    #[must_use]
    pub fn with_saver(mut self, saver: S) -> Self {
        self.saver = saver;
        self
    }


    /// Adds a list of characters [`chrs`] to the [`TrackerBuilder<S>`].
    #[must_use]
    pub fn with_chrs(mut self, chrs: impl Into<Vec<Chr>>) -> Self {
        let mut chrs: Vec<Chr> = chrs.into();
        chrs.sort();
        self.chrs = chrs;
        self
    }

    /// Builds a [`Tracker<S>`] from a [`TrackerBuilder<S>`].
    pub fn build(self) -> Tracker<S> {

        Tracker {
            chrs: self.chrs,
            in_turn_index: self.in_turn_index,
            saver: self.saver,
            cm: self.cm
        }
    }
}

impl<S: Saver> Tracker<S> {
    /// Creates a [`TrackerBuilder<S>`] for the purpose of the initialisation 
    /// of a [`Tracker<S>`].
    #[must_use]
    pub fn builder() -> Builder<S> {
        Builder::new(S::default())
    }

    /// Returns a reference to the character [`Chr`] with the given [`name`],
    /// if such a one exists.
    pub fn get_chr(&self, name: &str) -> Option<&Chr> {
        self.chrs.iter().find(|chr| chr.name == name)
    }

    /// Returns the position on the tracker order of the character with the
    /// given [`name`], if such a one exists.
    fn pos(&self, name: &str) -> Option<usize> {
        self.chrs.iter().enumerate().find(|(_,x)| x.name == name).map(|e| e.0)
    }

    /// Returns a reference to characters of this [`Tracker<S>`].
    pub fn get_chrs(&self) -> &[Chr] {
        &self.chrs[..]
    }

    /// Ends the turn and returns the new character in turn.
    /// If this [`Tracker<S>`] is empty, nothing happens on [`None`] is returned.
    ///
    /// # Errors
    ///
    /// This function will return an error if auto saving fails.
    pub fn end_turn(&mut self) -> Result<Option<&Chr>> {
        if let Some(chr) = self.get_in_turn().cloned() {
            let damage = self.cm.end_of_turn(&chr.name);
            if let Some(damage) = damage {
                // It can only fail if there is no character by the name,
                // which there naturally will always be in this if body
                self.damage(&chr.name, damage.into())?;
            }
        }

        if !self.chrs.is_empty() { 
            self.in_turn_index = Some(match self.in_turn_index {
                None => 0,
                Some(i) => (i + 1) % self.chrs.len(),
            });
        }

        if let Some(chr) = self.get_in_turn().cloned() {
            self.cm.start_of_turn(&chr.name);
        }
        self.auto_save()?;
        Ok(self.get_in_turn())
    }

    pub fn get_in_turn(&self) -> Option<&Chr> {
        self.in_turn_index.and_then(|i| self.chrs.get(i))
    }

    /// Adds a character [`chr`] to this [`Tracker<S>`].
    ///
    /// # Errors
    ///
    /// This function will return an error if auto saving fails.
    pub fn add_chr(&mut self, chr: Chr) -> Result<()> {
        if self.get_chr(&chr.name).is_some() { 
            return Err(Error::AddDupError(chr.name))
            // return Err(format!("Cannot add character {:?} since there is already a character by this name.", chr)) 
        }

        if let Some(i) = self.in_turn_index {
            if chr.init > self.chrs[i].init {
                self.in_turn_index = Some(i + 1);
            }
        }

        self.chrs.push(chr);
        self.chrs.sort();

        self.auto_save()?;

        Ok(())
    }

    /// Adds a [`Condition`] to the character named [`name`].
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - There is no character named [`name`]
    /// - Auto saving fails.
    pub fn add_condition(&mut self, name: &str, cond: Condition) -> Result<()> {
        match self.get_chr(name) {
            None => Err(Error::ChangeNoneError(name.to_string())),
            Some(_) => {
                self.cm.add_condition(name, cond);
                self.auto_save()?;
                Ok(())
            }
        }
    }

    /// Returns the conditions of a character with the given name.
    ///
    /// Provided the name of a character, a [`HashSet<&Condition>`] of
    /// their conditions is returned. If there is no character with the given name,
    /// an empty set is returned.
    pub fn get_conditions(&self, character: &str) -> HashSet<&Condition> {
        self.cm.get_conditions(character)
    }

    /// Removes the given condition type from the character with the givne name.
    ///
    /// If there is no character with the given name, or the character has no
    /// such condition, nothing happens.
    pub fn rm_condition(&mut self, character: &str, condition: &Condition) {
        self.cm.remove_condition(character, condition);
    }
    
    /// Removes a character with the given [`name`] from this [`Tracker<S>`].
    ///
    /// If the removed character is the one in turn, this ends the given
    /// characters turn.
    ///
    /// # Errors
    ///
    /// This function will return an error if 
    /// - There is no character with the given [`name`]
    /// - Auto saving fails.
    pub fn rm_chr(&mut self, name: &str) -> Result<()> {
        let rm_index = self.chrs.iter()
            .position(|chr| chr.name == name)
            .ok_or(Error::RmNoneError(name.to_string()))?;

        let removed = self.chrs.remove(rm_index);

        self.cm.remove_character(&removed.name);

        if self.chrs.is_empty() {
            self.in_turn_index = None;
            self.auto_save()?;
            return Ok(())
        } 

        if let Some(in_turn) = self.in_turn_index {
            match rm_index.cmp(&in_turn) {
                Ordering::Less => {
                    self.in_turn_index = Some(in_turn - 1);
                }
                Ordering::Equal => {
                    // While it may seem redundant to decrement `in_turn_index`
                    // only to then end the turn (which increments it), 
                    // ending the turn has other effects which should occur in this situation.
                    self.in_turn_index = in_turn.checked_sub(1);
                    self.end_turn()?;
                }
                Ordering::Greater => ()
            }
        }

        self.auto_save()?;

        Ok(())
    }

    /// Renames a the character named [`old`], giving it the name [`new`].
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - There's no character with the given [`name`] 
    /// - Auto saving fails.
    pub fn rename(&mut self, old: &str, new: impl Into<String>) -> Result<()> {
        let new: String = new.into();

        if self.chrs.iter().any(|chr| chr.name == new) {
            return Err(Error::RenameDupError { old: old.into(), new })
        }

        self.cm.rename_character(old, &new);

        self.unchecked_change(old, |chr| { chr.name = new; })
    }

    /// Changes the initiative of the character.
    ///
    /// Changes the initiative of the character named [`name`] to [`init`],
    /// and returns a [`MovedStatus`].
    ///
    /// # Errors
    ///
    /// This function will return an error if 
    /// - There's no character with the given [`name`]
    /// - Auto saving fails.
    pub fn change_init(&mut self, name: &str, init: i32) -> Result<Option<MovedStatus>> {
        self.change(name, |chr| chr.init = init)
    }

    /// Marks a character named as a player character.
    ///
    /// Marks the character given by the name [`name`] as a player character.
    ///
    /// # Errors
    ///
    /// This function will return an error if 
    /// - There's no character with the given [`name`]
    /// - Auto saving fails.
    pub fn set_player(&mut self, name: &str, player: bool) -> Result<()> {
        self.unchecked_change(name, |chr| chr.player = player)
    }

    /// Changes the max health of the character.
    ///
    /// Changes the max health of the character named [`name`] to [`max`].
    ///
    /// # Errors
    ///
    /// This function will return an error if 
    /// - There's no character with the given [`name`] 
    /// - Auto saving fails.
    pub fn change_max_health(&mut self, name: &str, max: u32) -> Result<()> {
        self.unchecked_change(name, |chr| {
            if let Some(hp) = &mut chr.health {
                hp.max = max;
                hp.current = hp.current.min(max);
            } else {
                chr.health = Some(Health::new(max));
            }
        })
    }

    /// Damages the character with the given [`name`] by the given [`amount`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    /// - There's no character with the given [`name`]
    /// - Auto saving fails.
    pub fn damage(&mut self, name: &str, amount: u32) -> Result<()> {
        self.unchecked_change(name, |chr| { chr.damage(amount); })
    }

    /// Heals the character with the given [`name`] by the given [`amount`].
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - There's no character wit hthe given [`name`]
    /// - Auto saving fails.
    pub fn heal(&mut self, name: &str, heal: u32) -> Result<()> {
        self.unchecked_change(name, |chr| { chr.heal(heal); })
    }

    fn unchecked_change<F>(&mut self, name: &str, f: F) -> Result<()> where
        F: FnOnce(&mut Chr)
    {
        for chr in &mut self.chrs {
            if chr.name == name {
                f(chr);
                self.chrs.sort();
                self.auto_save()?;
                return Ok(())
            }
        }

        Err(Error::ChangeNoneError(name.into()))
    }

    fn change<F>(&mut self, name: &str, f: F) -> Result<Option<MovedStatus>> where
        F: FnOnce(&mut Chr)
    {
        let before = self.pos(name).ok_or(Error::ChangeNoneError(name.into()))?;
        let in_turn = self.in_turn_index;

        self.unchecked_change(name, f)?;

        let after = self.pos(name).ok_or(Error::ChangeNoneError(name.into()))?;

        if let Some(in_turn) = in_turn {
            if before == in_turn && after < in_turn {
                return Ok(Some(MovedStatus::TwoTurns(self.chrs[after].clone())))
            }
            if before < in_turn && in_turn <= after  {
                self.in_turn_index = Some(in_turn - 1);
                return Ok(Some(MovedStatus::TwoTurns(self.chrs[after].clone())))
            } 
            if before > in_turn && in_turn >= after  {
                self.in_turn_index = Some(in_turn + 1);
                return Ok(Some(MovedStatus::Skipped(self.chrs[after].clone())))
            }                
        }

        Ok(None)
    }

    /// Saves this [`Tracker<S>`] to the file by the given [`file_name`].
    ///
    /// # Errors
    ///
    /// This function will return an error if [`saver.save`] fails.
    pub fn save(&self, file_name: impl Into<String>) -> Result<()> {
        let data: TrackerData = self.to_owned().into();
        self.saver.save(&data, format!("saves/{}", file_name.into()))?;
        Ok(())
    }

    fn auto_save(&self) -> Result<()> {
        self.save("auto.save")?;
        Ok(())
    }

    /// Loads a [`Tracker<S>`] from a file by the given [`file_name`].
    ///
    /// # Errors
    ///
    /// This function will return an error if [`saver.load`] fails.
    pub fn load(saver: &S, file_name: impl Into<String>) -> Result<Self> {
        let data: TrackerData = saver.load(format!("saves/{}", file_name.into()))?;
        let t: Tracker<S> = data.into();

        Ok(t)
    }
}
