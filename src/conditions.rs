use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

use crate::duration::Duration;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConditionManager {
    conds: HashSet<Condition>,
}

impl ConditionManager {
    pub fn new() -> Self {
        ConditionManager {
            conds: HashSet::new(),
        }
    }
    pub fn add(&mut self, cond: Condition) {
        let current = self.conds.get(&cond);

        if let Some(current) = current {
            if current.level < cond.level {
                self.conds.insert(cond);
            }
        } else {
            self.conds.insert(cond);
        }
    }

    pub fn remove(&mut self, affected: &str, cond_name: &str) {
        self.conds
            .retain(|cond| cond.name != cond_name || cond.affected != affected)
    }

    pub fn get<'a>(&'a self, character: &str) -> HashSet<&'a Condition> {
        self.conds
            .iter()
            .filter(|cond| cond.affected == character)
            .collect()
    }

    pub fn handle_cond_trigger(&mut self, trigger: CondTrigger) {
        let new_conds = self
            .conds
            .clone()
            .into_iter()
            .filter_map(|cond| match cond.cond_type {
                ConditionType::TimeBased(dur) => match trigger {
                    CondTrigger::EndOfTurn(_) => {
                        let new_dur = dur.saturating_sub(Duration::from_turns(1));
                        Some(Condition {
                            cond_type: ConditionType::TimeBased(new_dur),
                            ..cond
                        })
                    }
                    _ => Some(cond),
                },
                ConditionType::ReductionBased { reduction } => {
                    if cond.trigger == trigger {
                        println!("!!!");
                        reduction.map(|r| Condition {
                            level: cond.level.saturating_sub(r),
                            ..cond
                        })
                    } else {
                        println!("OH NO");
                        Some(cond)
                    }
                }
            })
            .collect();

        self.conds = new_conds
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub affected: String,
    pub level: u8,
    pub cond_type: ConditionType,
    pub trigger: CondTrigger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    TimeBased(Duration),
    ReductionBased { reduction: Option<u8> },
}

impl Hash for Condition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.affected.hash(state);
    }
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.affected == other.affected
    }
}

impl Eq for Condition {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CondTrigger {
    Manual { cond_name: String },
    StartOfTurn(String),
    EndOfTurn(String),
}
