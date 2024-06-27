use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::duration::Duration;

use super::{Condition, NonValuedTerm, TurnEvent, ValuedTerm};


#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct ConditionManager {
    conds: HashSet<(String,Condition)>,
}

impl ConditionManager {
    pub fn new() -> Self {
        ConditionManager {
            conds: HashSet::new(),
        }
    }
    pub fn add_condition(&mut self, character: &str, cond: Condition) {
        let exists_ge = self.get_conditions(character)
            .get(&cond)
            .map(|current| match (current,&cond) {
                (Condition::Valued { cond: c1, level: l1, .. }, 
                    Condition::Valued { cond: c2, level: l2, .. }) => c1 == c2 && l1.ge(l2),
                (Condition::NonValued { cond: c1, .. }, Condition::NonValued { cond: c2, .. }) => c1 == c2,
                _ => false
            })
            .unwrap_or(false);

        if !exists_ge {
            self.conds.insert((character.to_string(), cond));
        }
    }

    pub fn start_of_turn(&mut self, character: &str) {
        self.turn(character, TurnEvent::StartOfTurn(character.to_string()))
    }

    pub fn end_of_turn(&mut self, character: &str) {
        self.turn(character, TurnEvent::EndOfTurn(character.to_string()))
    }

    fn turn(&mut self, character: &str, event: TurnEvent) {
        let new_conds = self
            .conds
            .clone()
            .into_iter()
            .filter(|(affected, _)| affected == character)
            .filter_map(|(affected,cond)| match cond {
                Condition::Valued { term: ValuedTerm::Manual, .. } | 
                Condition::NonValued { term: NonValuedTerm::Manual, .. } => Some((affected,cond)),
                Condition::Valued { term: ValuedTerm::For(dur), cond, level } => {
                    let new_cond = Condition::Valued { term: ValuedTerm::For(dur.saturating_sub(Duration::from_turns(1))), cond, level };
                    Some((affected, new_cond))
                },
                Condition::NonValued { term: NonValuedTerm::For(dur), cond } => {
                    let new_cond = Condition::NonValued { term: NonValuedTerm::For(dur.saturating_sub(Duration::from_turns(1))), cond };
                    Some((affected, new_cond))
                }
                Condition::Valued { term: ValuedTerm::Until(e), .. } |
                Condition::NonValued { term: NonValuedTerm::Until(e), .. } if e == event => None,
                Condition::Valued { term: ValuedTerm::Until(_), .. } |
                Condition::NonValued { term: NonValuedTerm::Until(_), .. } => Some((affected,cond)),
                Condition::Valued { term: ValuedTerm::Reduced(e, reduction), level, cond }  if e == event => {
                    let new_cond = Condition::Valued { 
                        cond, 
                        term: ValuedTerm::Reduced(e, reduction), 
                        level: level.saturating_sub(reduction) 
                    };
                    Some((affected, new_cond))
                },
                Condition::Valued { term: ValuedTerm::Reduced(_,_), .. } => Some((affected, cond))
            })
            .collect();
        self.conds = new_conds;
    }

    pub fn remove_condition(&mut self, character: &str, condition: &Condition) {
        self.conds.retain(|(affected, cond)| affected != character || cond != condition)
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
        self.conds.iter()
            .filter(|(affected, _)| affected == character)
            .map(|(_, cond)| cond)
            .collect()
    }
}

