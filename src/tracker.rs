use std::{cmp::Ordering, collections::HashSet, ops::RangeBounds};

use thiserror::Error;

use crate::character::{Chr, Health};

#[derive(Debug, Error)]
#[derive(PartialEq)]
pub enum Error {
    #[error("cannot add character with name `{0}` as there is already a character with this name.")]
    AddDuplicateError(String),

    #[error("cannot remove a character of name `{0}` as no such character exists.")]
    RmNonexistentError(String),

    #[error("cannot modify character `{0}` as no such character exists.")]
    ChangeNonexistentError(String),

    #[error("cannot rename `{old}` into `{new}` as there is already a character with this name.")]
    RenameDuplicateError { old: String, new: String }
}

pub type TrackerResult = Result<(), Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Tracker {
    chrs: Vec<Chr>,
    in_turn_index: Option<usize>,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker::new(vec![])
    }
}

#[derive(Debug, PartialEq)]
#[derive(Clone)]
pub enum MovedStatus {
    Skipped(Chr),
    TwoTurns(Chr),
}

impl Tracker {
    pub fn new(chrs: impl Into<Vec<Chr>>) -> Self {
        let mut chrs: Vec<Chr> = chrs.into();
        chrs.sort();
        Tracker {
            chrs,
            in_turn_index: None
        }
    }

    pub fn get_chr(&self, name: &str) -> Option<&Chr> {
        self.chrs.iter().find(|chr| chr.name == name)
    }

    fn pos(&self, name: &str) -> Option<usize> {
        self.chrs.iter().enumerate().find(|(_,x)| x.name == name).map(|e| e.0)
    }

    pub fn get_chrs(&self) -> &[Chr] {
        &self.chrs[..]
    }

    pub fn end_turn(&mut self) -> Option<&Chr> {
        if self.chrs.is_empty() { return self.get_in_turn() }

        self.in_turn_index = Some(match self.in_turn_index {
            None => 0,
            Some(i) => (i + 1) % self.chrs.len(),
        });

        self.get_in_turn()
    }

    pub fn get_in_turn(&self) -> Option<&Chr> {
        self.in_turn_index.and_then(|i| self.chrs.get(i))
    }

    pub fn add_chr(&mut self, chr: Chr) -> TrackerResult {
        if self.get_chr(&chr.name).is_some() { 
            return Err(Error::AddDuplicateError(chr.name))
            // return Err(format!("Cannot add character {:?} since there is already a character by this name.", chr)) 
        }

        if let Some(i) = self.in_turn_index {
            if chr.init > self.chrs[i].init {
                self.in_turn_index = Some(i + 1)
            }
        }

        self.chrs.push(chr);
        self.chrs.sort();
        Ok(())
    }
    
    pub fn rm_chr(&mut self, name: &str) -> TrackerResult {
        let rm_index = self.chrs.iter()
            .position(|chr| chr.name == name)
            .ok_or(Error::RmNonexistentError(name.to_string()))?;

        self.chrs.remove(rm_index);

        if self.chrs.is_empty() {
            self.in_turn_index = None;
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
                    self.end_turn();
                }
                Ordering::Greater => ()
            }
        }

        Ok(())
    }

    pub fn rename(&mut self, old: &str, new: impl Into<String>) -> TrackerResult {
        let new: String = new.into();

        if self.chrs.iter().any(|chr| chr.name == new) {
            return Err(Error::RenameDuplicateError { old: old.into(), new })
        }

        self.unchecked_change(old, |chr| { chr.name = new; Ok(()) })
    }

    pub fn change_dex(&mut self, name: &str, dex: i32) -> Result<Option<MovedStatus>, Error> {
        self.change(name, |chr| { chr.dex = Some(dex); Ok(()) })
    }

    pub fn change_init(&mut self, name: &str, init: i32) -> Result<Option<MovedStatus>, Error> {
        self.change(name, |chr| { chr.init = init; Ok(()) })        
    }

    pub fn set_player(&mut self, name: &str, player: bool) -> TrackerResult {
        self.unchecked_change(name, |chr| { chr.player = player; Ok(()) })
    }

    pub fn change_max_health(&mut self, name: &str, health: u32) -> TrackerResult {
        self.unchecked_change(name, |chr| {
            if let Some(hp) = &mut chr.health {
                hp.max = health;
            } else {
                chr.health = Some(Health::new(health));
            }
            Ok(())
        })
    }

    pub fn damage(&mut self, name: &str, damage: u32) -> TrackerResult {
        self.unchecked_change(name, |chr| { chr.damage(damage); Ok(()) })
    }

    pub fn heal(&mut self, name: &str, heal: u32) -> TrackerResult {
        self.unchecked_change(name, |chr| { chr.heal(heal); Ok(()) })
    }

    fn unchecked_change<F>(&mut self, name: &str, f: F) -> TrackerResult where
        F: FnOnce(&mut Chr) -> TrackerResult
    {
        for chr in &mut self.chrs {
            if chr.name == name {
                return f(chr).and({
                    self.chrs.sort();
                    Ok(())
                })
            }
        }

        Err(Error::ChangeNonexistentError(name.into()))
    }

    fn change<F>(&mut self, name: &str, f: F) -> Result<Option<MovedStatus>, Error> where
        F: FnOnce(&mut Chr) -> TrackerResult
    {
        let before = self.pos(name).ok_or(Error::ChangeNonexistentError(name.into()))?;
        let in_turn = self.in_turn_index;

        self.unchecked_change(name, f)?;

        let after = self.pos(name).unwrap();

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
}
