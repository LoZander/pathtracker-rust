use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::duration::Duration;

use super::{Condition, NonValuedTerm, TurnEvent, ValuedCondition, ValuedTerm};

pub type Damage = u8;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct ConditionManager {
    conds: HashSet<(String,Condition)>,
}

impl ConditionManager {
    #[must_use]
    pub fn new() -> Self {
        ConditionManager {
            conds: HashSet::new(),
        }
    }
    pub fn add_condition(&mut self, character: &str, cond: Condition) {
        let exists_ge = self.get_conditions(character)
            .get(&cond)
            .is_some_and(|current| match (current,&cond) {
                (Condition::Valued { cond: c1, level: l1, .. }, 
                    Condition::Valued { cond: c2, level: l2, .. }) => c1 == c2 && l1.ge(l2),
                (Condition::NonValued { cond: c1, .. }, Condition::NonValued { cond: c2, .. }) => c1 == c2,
                _ => false
            });

        if !exists_ge {
            self.conds.insert((character.to_string(), cond));
        }
    }

    pub fn start_of_turn(&mut self, character: &str) {
        self.handle_turn_event(&TurnEvent::StartOfTurn(character.to_string()));
    }

    pub fn end_of_turn(&mut self, character: &str) -> Option<Damage> {
        let damage = self.get_conditions(character).iter()
            .filter_map(|cond| match cond {
                Condition::Valued { cond: ValuedCondition::PersistentDamage(_), level, .. } => Some(*level),
                _ => None
            })
            .sum();

        self.handle_turn_event(&TurnEvent::EndOfTurn(character.to_string()));

        match damage {
            0 => None,
            d => Some(d)
        }
    }

    fn handle_turn_event(&mut self, event: &TurnEvent) {
        let new_conds = self
            .conds
            .clone()
            .into_iter()
            .filter_map(|(affected, cond)| {
                match (affected, cond) {
                    (affected, condition @ Condition::Valued { term: ValuedTerm::For(dur), cond, level }) => {
                        match &event {
                            TurnEvent::EndOfTurn(c) if c == &affected => {
                                match dur.in_turns() {
                                    0 | 1 => None,
                                    n => {
                                        let new_dur = Duration::from_turns(n - 1);
                                        let new_cond = Condition::Valued {
                                            term: ValuedTerm::For(new_dur),
                                            level,
                                            cond,
                                        };
                                        Some((affected, new_cond))
                                    }
                                }
                            },
                            _ => Some((affected, condition))
                        }
                    },
                    (affected, condition @ Condition::NonValued { term: NonValuedTerm::For(dur), cond }) => {
                        match &event {
                            TurnEvent::EndOfTurn(c) if c == &affected => {
                                match dur.in_turns() {
                                    0 | 1 => None,
                                    n => {
                                        let new_dur = Duration::from_turns(n - 1);
                                        let new_cond = Condition::NonValued {
                                            term: NonValuedTerm::For(new_dur),
                                            cond,
                                        };
                                        Some((affected, new_cond))
                                    }
                                }
                            },
                            _ => Some((affected, condition))
                        }
                    }
                    (_, Condition::Valued { term: ValuedTerm::Until(e), .. } |
                        Condition::NonValued { term: NonValuedTerm::Until(e), .. }) if &e == event => None,
                    (affected, Condition::Valued { term: ValuedTerm::Reduced(e, reduction), level, cond }) if &e == event => {
                        let new_cond = Condition::Valued { 
                            cond, 
                            term: ValuedTerm::Reduced(e, reduction), 
                            level: level.saturating_sub(reduction) 
                        };
                        Some((affected, new_cond))
                    },
                    (affected, cond) => Some((affected, cond))
                }
            })
            .collect();
        self.conds = new_conds;
    }

    pub fn remove_condition(&mut self, character: &str, condition: &Condition) {
        self.conds.retain(|(affected, cond)| affected != character || cond != condition);
    }

    pub fn rename_character(&mut self, character: &str, new_name: &str) {
        let conds = self.conds.clone().into_iter()
            .map(|(affected, cond)| {
                if affected == character {
                    (new_name.to_owned(), cond)
                } else {
                    (affected, cond)
                }
            })
            .collect();
        self.conds = conds;
    }

    pub fn remove_character(&mut self, character: &str) {
        self.conds.retain(|(affected, _)| affected != character);
    }

    #[must_use]
    pub fn get_conditions<'a>(&'a self, character: &str) -> HashSet<&'a Condition> {
        self.conds.iter()
            .filter(|(affected, _)| affected == character)
            .map(|(_, cond)| cond)
            .collect()
    }
}

