use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

use crate::duration::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub level: u8,
    pub cond_type: ConditionType,
}


impl Hash for Condition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Condition {}


#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConditionManager {
    conds: HashSet<(String, Condition)>,
}

impl ConditionManager {
    pub fn new() -> Self {
        ConditionManager {
            conds: HashSet::new(),
        }
    }
    pub fn add_conditions(&mut self, character: &str, cond: Condition) {
        let current = self.conds.get(&(character.to_string(), cond.clone()));

        if let Some((_, cur_cond)) = current {
            if cur_cond.level < cond.level {
                self.conds.insert((character.to_string(), cond));
            }
        } else {
            self.conds.insert((character.to_string(), cond));
        }
    }

    pub fn start_of_turn(&mut self, character: &str) {
        self.turn(character, start_of_turn_reduction_handler)
    }

    pub fn end_of_turn(&mut self, character: &str) {
        self.turn(character, end_of_turn_reduction_handler)
    }

    fn turn(&mut self, character: &str, reductionbased_handle: impl Fn(&str, Condition, Option<u8>, &ReductionTrigger) -> Option<Condition>) {
        let new_conds = self
            .conds
            .clone()
            .into_iter()
            .filter(|(affected, _)| affected == character)
            .filter_map(|(affected, cond)| match cond.cond_type {
                ConditionType::Manual => Some((affected, cond)),
                ConditionType::TimeBased(dur) => handle_timebased_condition(cond, dur).map(|cond| (affected, cond)),
                ConditionType::ReductionBased { reduction, ref trigger } => reductionbased_handle(character, cond.clone(), reduction, trigger).map(|cond| (affected, cond)),
            })
            .collect();
        self.conds = new_conds
    }

    pub fn remove_condition(&mut self, character: &str, cond_name: &str) {
        self.conds
            .retain(|(affected, cond)| affected != character || cond.name != cond_name)
    }

    pub fn rename_character(&mut self, character: &str, new_name: impl Into<String>) {
        let new_name: String = new_name.into();
        let conds = self.conds.clone().into_iter()
            .map(|(affected, cond)| {
                if affected == character {
                    (new_name.clone(), cond)
                } else {
                    (affected, cond)
                }
            })
            .collect();
        self.conds = conds
    }

    pub fn remove_character(&mut self, character: &str) {
        self.conds
            .retain(|(affected, _)| affected != character)
    }

    pub fn get_conditions<'a>(&'a self, character: &str) -> HashSet<&'a Condition> {
        self.conds
            .iter()
            .filter(|(affected, _)| affected == character)
            .map(|(_, cond)| cond)
            .collect()
    }
}

fn handle_timebased_condition(cond: Condition, duration: Duration) -> Option<Condition> {
    let new_duration = duration.saturating_sub(Duration::from_turns(1));
    Some(Condition { cond_type: ConditionType::TimeBased(new_duration), ..cond })
}

fn start_of_turn_reduction_handler(character: &str, cond: Condition, reduction: Option<u8>, trigger: &ReductionTrigger) -> Option<Condition> {
    match trigger {
        ReductionTrigger::StartOfTurn(other) if character == other => {
            reduction.map(|r| Condition { level: cond.level.saturating_sub(r), ..cond })
        },
        _ => Some(cond)
    }
}

fn end_of_turn_reduction_handler(character: &str, cond: Condition, reduction: Option<u8>, trigger: &ReductionTrigger) -> Option<Condition> {
    match trigger {
        ReductionTrigger::EndOfTurn(other) if character == other => {
            reduction.map(|r| Condition { level: cond.level.saturating_sub(r), ..cond })
        },
        _ => Some(cond)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    Manual,
    TimeBased(Duration),
    ReductionBased {
        reduction: Option<u8>,
        trigger: ReductionTrigger,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ReductionTrigger {
    StartOfTurn(String),
    EndOfTurn(String)
}
