use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{character::ChrName, duration::Duration};

use super::{Condition, NonValuedCondition, NonValuedTerm, TurnEvent, ValuedCondition, ValuedTerm};

pub type Damage = u8;

/// Manages the conditions of characters in the tracker.
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

    /// Signals the start of a character's turn to the condition manager.
    pub fn start_of_turn(&mut self, character: ChrName) {
        self.handle_turn_event(&TurnEvent::StartOfNextTurn(character));
        self.new_conds.clear();
    }

    /// Removes a given condition from a specific character if they have it.
    ///
    /// If the character does not have the condition, nothing changes.
    pub fn remove_condition(&mut self, character: &ChrName, condition: &Condition) {
        self.conds.retain(|(affected, cond)| affected != character || cond != condition);
    }

    /// Renames a character in the condition manager
    #[allow(clippy::needless_pass_by_value)]
    pub fn rename_character(&mut self, character: &ChrName, new_name: ChrName) {
        let conds = self.conds.clone().into_iter()
            .map(|(affected, cond)| {
                if affected == character {
                    (new_name.clone(), cond)
                } else {
                    (affected, cond)
                }
            })
            .collect();
        self.conds = conds;
    }

    /// Removes a character from the condition manager.
    pub fn remove_character(&mut self, character: &ChrName) {
        self.conds.retain(|(affected, _)| affected != character);
    }

    /// Returns the given character's conditions.
    #[must_use]
    pub fn get_conditions<'a>(&'a self, character: &ChrName) -> HashSet<&'a Condition> {
        self.conds.iter()
            .filter(|(affected, _)| affected == character)
            .map(|(_, cond)| cond)
            .collect()
    }

    /// Signals the end of a character's turn to the condition manager.
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
            .filter_map(|(affected, cond)| self.cond_step(event, affected, cond))
            .collect();
        self.conds = new_conds;
    }

    fn cond_step(&self, event: &TurnEvent, affected: ChrName, cond: Condition) -> Option<(ChrName, Condition)> {
        match cond {
            Condition::Valued { term: ValuedTerm::For(dur), cond, level } =>
                cond_step_for_valued(event, affected, dur, cond, level),
            Condition::NonValued { term: NonValuedTerm::For(dur), cond } =>
                cond_step_for_nonvalued(event, affected, dur, cond),
            Condition::Valued { term: ValuedTerm::Until(ref e @ TurnEvent::EndOfCurrentTurn(_)), .. } |
            Condition::NonValued { term: NonValuedTerm::Until(ref e @ TurnEvent::EndOfCurrentTurn(_)), .. }
            if e == event =>
                None,
            ref condition @
            (Condition::Valued { term: ValuedTerm::Until(ref e), .. } |
            Condition::NonValued { term: NonValuedTerm::Until(ref e), .. })
            if e == event && self.has_new_condition_on(&affected, condition) => 
                None,
            Condition::Valued { term: ValuedTerm::Reduced(e, reduction), level, cond }
            if &e == event =>
                self.cond_step_reduced(affected, e, reduction, level, cond),
            cond => 
                Some((affected, cond))
        }
    }

    fn cond_step_reduced(&self, affected: ChrName, event: TurnEvent, reduction: u8, level: u8, cond: ValuedCondition) -> Option<(ChrName, Condition)> {
        match event {
            TurnEvent::StartOfNextTurn(_) |
            TurnEvent::EndOfCurrentTurn(_) => {
                reduce(reduction, level)
                    .map(|l| Condition::builder().condition(cond).value(l).term(ValuedTerm::Reduced(event, reduction)).build())
                    .map(|condition| (affected, condition))
            }
            // TurnEvent::EndOfNextTurn(_) if self.has_new_condition_on(&affected, &Condition::Valued { term: ValuedTerm::Reduced(event.clone(), reduction), level, cond }) => {
            TurnEvent::EndOfNextTurn(_) if self.has_new_condition_on(&affected, &Condition::builder().condition(cond).value(level).term(ValuedTerm::Reduced(event.clone(), reduction)).build()) => {
                reduce(reduction, level)
                    .map(|l| Condition::builder().condition(cond).value(l).term(ValuedTerm::Reduced(event, reduction)).build())
                    .map(|condition| (affected, condition))
            },
            TurnEvent::EndOfNextTurn(_) => Some((affected, Condition::builder().condition(cond).value(level).term(ValuedTerm::Reduced(event, reduction)).build()))
        }
    }

    fn has_new_condition_on(&self, affected: &ChrName, condition: &Condition) -> bool {
        !self.new_conds.iter().any(|(a, b)| a == affected && b == condition)
    }

}

const fn reduce(reduction: u8, level: u8) -> Option<u8> {
    let new_level = level.saturating_sub(reduction);
    match new_level {
        0 => None,
        l => Some(l)
    }
}

fn duration_turn(dur: Duration) -> Option<Duration> {
    match dur.in_turns() {
        0 | 1 => None,
        n => Some(Duration::from_turns(n - 1))
    }
}


fn cond_step_for_nonvalued(event: &TurnEvent, affected: ChrName, dur: Duration, cond: NonValuedCondition) -> Option<(ChrName, Condition)> {
    match &event {
        TurnEvent::EndOfNextTurn(c) if c == affected => {
            duration_turn(dur)
                .map(NonValuedTerm::For)
                .map(|term| Condition::builder().condition(cond).term(term).build())
                .map(|condition| (affected, condition))
        },
        _ => Some((affected, Condition::builder().condition(cond).term(NonValuedTerm::For(dur)).build()))
    }
}

fn cond_step_for_valued(event: &TurnEvent, affected: ChrName, dur: Duration, cond: ValuedCondition, level: u8) -> Option<(ChrName, Condition)> {
    match &event {
        TurnEvent::EndOfNextTurn(c) if c == affected => {
            duration_turn(dur)
                .map(ValuedTerm::For)
                .map(|term| Condition::builder().condition(cond).value(level).term(term).build())
                .map(|condition| (affected, condition))
        },
        _ => Some((affected, Condition::Valued { term: ValuedTerm::For(dur), cond, level }))
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

        cm.add_condition(jevil.name, dazzled.clone());

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
        cm.end_of_turn(jevil.name);

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
        cm.end_of_turn(jevil.name);

        assert!(!cm.get_conditions(&chris.name).contains(&dazzled));
    }
}

