use std::cmp::Ordering;

use thiserror::Error;

use crate::character::Chr;

#[derive(Debug, Error)]
pub enum Error {
    #[error("cannot add character with name `{0}` as there is already a character with this name")]
    AddDuplicateError(String),

    #[error("cannot remove a character of name `{0}` as no such character exists")]
    RmNonexistentError(String)
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

impl Tracker {
    pub fn new(chrs: impl Into<Vec<Chr>>) -> Self {
        let mut chrs: Vec<Chr> = chrs.into();
        chrs.sort();
        Tracker {
            chrs,
            in_turn_index: None
        }
    }

    #[allow(dead_code)]
    pub fn get_chr(&self, name: &str) -> Option<&Chr> {
        self.chrs.iter().find(|chr| chr.name == name)
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

    #[allow(dead_code)]
    pub fn get_chrs(&self) -> &[Chr] {
        &self.chrs[..]
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
}



#[cfg(test)]
mod tests {
    use super::{Chr, Tracker, TrackerResult};

    #[test]
    fn add_player_chr_alison_adds_chr() -> TrackerResult {
        let mut t = Tracker::new(vec![]);
        t.add_chr(Chr::builder("Alison", 21, true).build())?;
        assert_eq!(Some(&Chr::builder("Alison", 21, true).build()), t.get_chr("Alison"));
        Ok(())
    }

    #[test]
    fn initial_chrs_have_descending_initiative_order() {
        let mut t = Tracker::new(vec![
            Chr::builder("Skelly Boy", 3, false).build(),
            Chr::builder("Bucky", 30, true).build(),
            Chr::builder("Hellen", 27, true).build(),
        ]);

        assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.end_turn());
        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.end_turn());
        assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.end_turn())
    }

    #[test]
    fn add_preserves_descending_inititative_order() -> TrackerResult {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
            Chr::builder("Hellen", 27, true).build(),
            Chr::builder("Skelly Boy", 3, false).build(),
        ]);

        
        t.add_chr(Chr::builder("Kristy", 24, true).build())?;

        assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.end_turn());
        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.end_turn());
        assert_eq!(Some(&Chr::builder("Kristy", 24, true).build()), t.end_turn());
        assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.end_turn());

        Ok(())
    }

    #[test]
    fn end_turn_loops_around() {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
            Chr::builder("Hellen", 27, true).build(),
            Chr::builder("Skelly Boy", 3, false).build(),
        ]);

        t.end_turn();
        t.end_turn();

        assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.end_turn());
        assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.end_turn())
    }

    #[test]
    fn add_chr_before_in_turn_preserves_in_turn() -> TrackerResult {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
            Chr::builder("Hellen", 27, true).build(),
            Chr::builder("Skelly Boy", 3, false).build(),
        ]);

        t.end_turn();
        t.end_turn();

        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

        t.add_chr(Chr::builder("Lucky", 28, false).build())?;
        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

        Ok(())
    }

    #[test]
    fn add_chr_after_in_turn_preserves_in_turn() -> TrackerResult {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
            Chr::builder("Hellen", 27, true).build(),
            Chr::builder("Skelly Boy", 3, false).build(),
        ]);

        t.end_turn();
        t.end_turn();

        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());
        t.add_chr(Chr::builder("Unlucky", 24, false).build())?;
        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

        Ok(())
    }

    #[test]
    fn rm_chr_before_in_turn_preserves_in_turn() -> TrackerResult {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
            Chr::builder("Hellen", 27, true).build(),
            Chr::builder("Skelly Boy", 3, false).build(),
        ]);

        t.end_turn();
        t.end_turn();

        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());
        t.rm_chr("Bucky")?;
        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

        Ok(())
    }

    #[test]
    fn rm_chr_after_in_turn_preserves_in_turn() -> TrackerResult {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
            Chr::builder("Hellen", 27, true).build(),
            Chr::builder("Skelly Boy", 3, false).build(),
        ]);

        t.end_turn();
        t.end_turn();

        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());
        t.rm_chr("Skelly Boy")?;
        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

        Ok(())
    }

    #[test]
    fn rm_only_chr_in_turn_makes_no_one_in_turn() -> TrackerResult {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
        ]);

        t.end_turn();

        assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.get_in_turn());
        t.rm_chr("Bucky")?;
        assert_eq!(None, t.get_in_turn());

        Ok(())
    }

    #[test]
    fn rm_only_chr_makes_no_one_in_turn() -> TrackerResult {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
        ]);

        assert_eq!(None, t.get_in_turn());
        t.rm_chr("Bucky")?;
        assert_eq!(None, t.get_in_turn());

        Ok(())
    }

    #[test]
    fn rm_in_turn_makes_it_next_ups_turn() -> TrackerResult {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
            Chr::builder("Hellen", 27, true).build(),
            Chr::builder("Skelly Boy", 3, false).build(),
        ]);

        t.end_turn();
        t.end_turn();

        assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());
        t.rm_chr("Hellen")?;
        assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.get_in_turn());

        Ok(())
    }

    #[test]
    fn rm_last_when_its_turn_makes_it_top_of_round() -> TrackerResult {
        let mut t = Tracker::new(vec![
            Chr::builder("Bucky", 30, true).build(),
            Chr::builder("Hellen", 27, true).build(),
            Chr::builder("Skelly Boy", 3, false).build(),
        ]);

        t.end_turn();
        t.end_turn();
        t.end_turn();

        assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.get_in_turn());
        t.rm_chr("Skelly Boy")?;
        assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.get_in_turn());

        Ok(())
    }

    #[test]
    fn end_turn_when_no_chrs_makes_it_no_ones_turn() {
        let mut t = Tracker::new(vec![]);
        assert!(t.end_turn().is_none())
    }
}
