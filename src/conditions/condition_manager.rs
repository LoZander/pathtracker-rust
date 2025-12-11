use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{character::ChrName, duration::Duration};

use super::{Condition, NonValuedTerm, TurnEvent, ValuedCondition, ValuedTerm};

pub type Damage = u8;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct ConditionManager {
    conds: HashSet<(ChrName,Condition)>,
    new_conds: HashSet<(ChrName,Condition)>,
}

impl ConditionManager {
    #[must_use]
    pub fn new() -> Self {
        Self { conds: HashSet::new(), new_conds: HashSet::new() }
    }
    pub fn add_condition(&mut self, character: ChrName, cond: Condition) {
        let exists_ge = self.get_conditions(&character)
            .get(&cond)
            .is_some_and(|current| match (current,&cond) {
                (Condition::Valued { cond: c1, level: l1, .. }, 
                    Condition::Valued { cond: c2, level: l2, .. }) => c1 == c2 && l1.ge(l2),
                (Condition::NonValued { cond: c1, .. }, Condition::NonValued { cond: c2, .. }) => c1 == c2,
                _ => false
            });

        if !exists_ge {
            self.conds.insert((character.clone(), cond.clone()));
            // Since `new_conds` is only needed to avoid
            // ending `Until` conditions that were added to a
            // character during their own turn,
            // we technically only need to add the new conditions
            // added to the character whose turn it is and not
            // every new condition. It's not wrong to add every new condition
            // as long as we only check against those affecting the character
            // in question, but it's unnecessary to add them all.
            self.new_conds.insert((character, cond));
        }
    }

    pub fn start_of_turn(&mut self, character: ChrName) {
        self.handle_turn_event(&TurnEvent::StartOfNextTurn(character));
        self.new_conds.clear();
    }

    pub fn end_of_turn(&mut self, character: ChrName) -> Option<Damage> {
        let damage = self.get_conditions(&character).iter()
            .filter_map(|cond| match cond {
                Condition::Valued { cond: ValuedCondition::PersistentDamage(_), level, .. } => Some(*level),
                _ => None
            })
            .sum();

        self.handle_turn_event(&TurnEvent::EndOfCurrentTurn(character.clone()));
        self.handle_turn_event(&TurnEvent::EndOfNextTurn(character));

        self.new_conds.clear();

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
                            TurnEvent::EndOfNextTurn(c) if c == &affected => {
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
                            TurnEvent::EndOfNextTurn(c) if c == &affected => {
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
                    (affected, ref condition @ 
                        (Condition::Valued { term: ValuedTerm::Until(ref e @ TurnEvent::EndOfCurrentTurn(_)), .. } |
                        Condition::NonValued { term: NonValuedTerm::Until(ref e @ TurnEvent::EndOfCurrentTurn(_)), .. }))
                    if e == event => None,
                    (affected, ref condition @
                        (Condition::Valued { term: ValuedTerm::Until(ref e), .. } |
                        Condition::NonValued { term: NonValuedTerm::Until(ref e), .. }))
                    if e == event && self.has_new_condition_on(&affected, condition) => None,
                    (affected, Condition::Valued { term: ValuedTerm::Reduced(e @ TurnEvent::EndOfCurrentTurn(_), reduction), level, cond })
                    if &e == event => {
                        let new_level = level.saturating_sub(reduction);
                        match new_level {
                            0 => None,
                            level => {
                                let new_cond = Condition::Valued { 
                                    cond, 
                                    term: ValuedTerm::Reduced(e, reduction), 
                                    level 
                                };
                                Some((affected, new_cond))
                            }
                        }
                    },
                    (affected, Condition::Valued { term: ValuedTerm::Reduced(e, reduction), level, cond })
                    if &e == event && self.has_new_condition_on(&affected, &Condition::Valued { term: ValuedTerm::Reduced(e.clone(), reduction), level, cond }) => {
                        let new_level = level.saturating_sub(reduction);
                        match new_level {
                            0 => None,
                            level => {
                                let new_cond = Condition::Valued { 
                                    cond, 
                                    term: ValuedTerm::Reduced(e, reduction), 
                                    level 
                                };
                                Some((affected, new_cond))
                            }
                        }
                    },
                    (affected, cond) => Some((affected, cond))
                }
            })
            .collect();
        self.conds = new_conds;
    }

    fn has_new_condition_on(&self, affected: &ChrName, condition: &Condition) -> bool {
        !self.new_conds.iter().any(|(a, b)| a == affected && b == condition)
    }

    pub fn remove_condition(&mut self, character: &ChrName, condition: &Condition) {
        self.conds.retain(|(affected, cond)| affected != character || cond != condition);
    }

    pub fn rename_character(&mut self, character: &ChrName, new_name: ChrName) {
        let conds = self.conds.clone().into_iter()
            .map(|(affected, cond)| {
                if &affected == character {
                    (new_name.clone(), cond)
                } else {
                    (affected, cond)
                }
            })
            .collect();
        self.conds = conds;
    }

    pub fn remove_character(&mut self, character: &ChrName) {
        self.conds.retain(|(affected, _)| affected != character);
    }

    #[must_use]
    pub fn get_conditions<'a>(&'a self, character: &ChrName) -> HashSet<&'a Condition> {
        self.conds.iter()
            .filter(|(affected, _)| affected == character)
            .map(|(_, cond)| cond)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{character::Chr, conditions::{condition_manager::ConditionManager, Condition, NonValuedCondition, NonValuedTerm, TurnEvent}};

    #[test]
    fn add_conditions_adds_it() {
        let mut cm = ConditionManager::new();

        let jevil = Chr::builder("Jevil", 24, false).build();

        let jevil_turn_ends = TurnEvent::EndOfNextTurn(jevil.name.clone());

        let dazzled = Condition::builder()
            .condition(NonValuedCondition::Dazzled)
            .term(NonValuedTerm::Until(jevil_turn_ends))
            .build();

        cm.add_condition(jevil.name.clone(), dazzled.clone());

        assert!(cm.get_conditions(&jevil.name).contains(&dazzled));
    }

    #[test]
    fn add_condition_doesnt_add_it_to_others() {
        let mut cm = ConditionManager::new();

        let jevil = Chr::builder("Jevil", 24, false).build();
        let chris = Chr::builder("Chris", 19, true).build();

        let jevil_turn_ends = TurnEvent::EndOfNextTurn(jevil.name.clone());

        let dazzled = Condition::builder()
            .condition(NonValuedCondition::Dazzled)
            .term(NonValuedTerm::Until(jevil_turn_ends))
            .build();

        cm.add_condition(jevil.name.clone(), dazzled.clone());

        assert!(!cm.get_conditions(&chris.name).contains(&dazzled));
    }

    #[test]
    fn until_end_of_next_turn_doesnt_end_if_next_event_is_end_of_turn() {
        let mut cm = ConditionManager::new();

        let jevil = Chr::builder("Jevil", 24, false).build();

        let jevil_turn_ends = TurnEvent::EndOfNextTurn(jevil.name.clone());

        let dazzled = Condition::builder()
            .condition(NonValuedCondition::Dazzled)
            .term(NonValuedTerm::Until(jevil_turn_ends))
            .build();

        cm.add_condition(jevil.name.clone(), dazzled.clone());
        cm.end_of_turn(jevil.name.clone());

        assert!(cm.get_conditions(&jevil.name).contains(&dazzled));
    }

    #[test]
    fn until_end_of_next_turn_ends_if_another_event_and_then_end_of_turn() {
        let mut cm = ConditionManager::new();

        let jevil = Chr::builder("Jevil", 24, false).build();

        let jevil_turn_ends = TurnEvent::EndOfNextTurn(jevil.name.clone());

        let dazzled = Condition::builder()
            .condition(NonValuedCondition::Dazzled)
            .term(NonValuedTerm::Until(jevil_turn_ends))
            .build();

        cm.add_condition(jevil.name.clone(), dazzled.clone());
        cm.start_of_turn(jevil.name.clone());
        cm.end_of_turn(jevil.name.clone());

        assert!(!cm.get_conditions(&jevil.name).contains(&dazzled));
    }

    #[test]
    fn until_end_of_next_turn_doesnt_end_if_end_of_turn_when_affected_is_different() {
        let mut cm = ConditionManager::new();

        let jevil = Chr::builder("Jevil", 24, false).build();
        let chris = Chr::builder("Chris", 20, true).build();

        let jevil_turn_ends = TurnEvent::EndOfNextTurn(jevil.name.clone());

        let dazzled = Condition::builder()
            .condition(NonValuedCondition::Dazzled)
            .term(NonValuedTerm::Until(jevil_turn_ends))
            .build();

        cm.add_condition(chris.name.clone(), dazzled.clone());
        cm.end_of_turn(jevil.name.clone());

        assert!(cm.get_conditions(&chris.name).contains(&dazzled));
    }

    #[test]
    fn until_end_of_next_turn_ends_if_another_event_and_then_end_of_turn_when_affected_is_different() {
        let mut cm = ConditionManager::new();

        let jevil = Chr::builder("Jevil", 24, false).build();
        let chris = Chr::builder("Chris", 20, true).build();

        let jevil_turn_ends = TurnEvent::EndOfNextTurn(jevil.name.clone());

        let dazzled = Condition::builder()
            .condition(NonValuedCondition::Dazzled)
            .term(NonValuedTerm::Until(jevil_turn_ends))
            .build();

        cm.add_condition(chris.name.clone(), dazzled.clone());
        cm.start_of_turn(jevil.name.clone());
        cm.end_of_turn(jevil.name.clone());

        assert!(!cm.get_conditions(&chris.name).contains(&dazzled));
    }
}

