use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::duration::Duration;

pub mod condition_manager;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Condition {
    Valued { cond: ValuedCondition, term: ValuedTerm, level: u8 },
    NonValued { cond: NonValuedCondition, term: NonValuedTerm }
}

impl Condition {
    pub fn builder() -> ConditionBuilder {
        ConditionBuilder::default()
    }
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Condition::Valued { cond: c1, .. }, Condition::Valued { cond: c2, .. }) => c1 == c2,
            (Condition::NonValued { cond: c1, .. }, Condition::NonValued { cond: c2, .. }) => c1 == c2,
            _ => false
        }
    }
}

impl Eq for Condition {}

impl Hash for Condition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Condition::Valued { cond, .. } => cond.hash(state),
            Condition::NonValued { cond, .. } => cond.hash(state),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Hash)]
pub enum TurnEvent {
    StartOfTurn(String),
    EndOfTurn(String)
}


#[derive(Debug, Clone, Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize, Hash)]
pub enum NonValuedTerm {
    #[default]
    Manual,
    For(Duration),
    Until(TurnEvent)
}

#[derive(Debug, Clone, Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize, Hash)]
pub enum ValuedTerm {
    #[default]
    Manual,
    For(Duration),
    Until(TurnEvent),
    Reduced(TurnEvent, u8)
}



#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Hash)]
pub enum ValuedCondition {
    PersistentDamage(DamageType),
    Clumsy,    
    Doomed,
    Drained,
    Dying,
    Enfeebled,
    Frightened,
    Sickened,
    Slowed,
    Stunned,
    Stupified,
    Wounded,
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Hash)]
pub enum NonValuedCondition {
    Blinded,
    Broken,
    Concealed,
    Confused,
    Controlled,
    Dazzled,
    Deafened,
    Encumbered, // just makes you clumsy 1
    Fascinated,
    Fatigued,
    FlatFooted,
    Fleeing,
    Friendly,
    Grabbed,
    Helpful,
    Hidden,
    Hostile,
    Immobilized,
    Indifferent,
    Invisible,
    Observed,
    Paralyzed,
    Petrified,
    Prone,
    Quickened,
    Restrained,
    Unconscious,
    Undetected,
    Unfriendly,
    Unnoticed
}


#[derive(Debug, Clone, Copy, Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Hash)]
pub enum DamageType {
    #[default]
    Bleed,
    Poison,
    Piercing,
    Bludgeoning,
    Slashing,
    Acid,
    Cold,
    Electricity,
    Sonic,
    Positive,
    Negative,
    Force,
    Chaotic,
    Evil,
    Good,
    Lawful
}


pub struct Empty;
pub trait CondType {}

impl CondType for ValuedCondition {}
impl CondType for NonValuedCondition {}

pub struct ConditionBuilder<Cond=Empty,Value=Empty,Term=Empty> {
    cond: Cond,
    value: Value,
    term: Term
}

impl Default for ConditionBuilder<Empty,Empty,Empty> {
    fn default() -> Self {
        Self { cond: Empty, value: Empty, term: Empty }
    }
}

impl ConditionBuilder<Empty,Empty,Empty> {
    pub fn condition<Cond: CondType>(self, cond: Cond) -> ConditionBuilder<Cond,Empty,Empty> {
        ConditionBuilder {
            cond,
            value: Empty,
            term: Empty,
        }
    }
}

impl<Term> ConditionBuilder<NonValuedCondition,Empty,Term> {
    pub fn term(self, term: NonValuedTerm) -> ConditionBuilder<NonValuedCondition,Empty,NonValuedTerm> {
        ConditionBuilder {
            cond: self.cond,
            value: self.value,
            term,
        }
    }
}

impl ConditionBuilder<NonValuedCondition,Empty,NonValuedTerm> {
    pub fn build(self) -> Condition {
        Condition::NonValued {
            cond: self.cond,
            term: self.term,
        }
    }
}

impl ConditionBuilder<NonValuedCondition,Empty,Empty> {
    pub fn build(self) -> Condition {
        Condition::NonValued {
            cond: self.cond,
            term: NonValuedTerm::default()
        }
    }
}


impl<Value,Term> ConditionBuilder<ValuedCondition,Value,Term> {
    pub fn value(self, value: u8) -> ConditionBuilder<ValuedCondition,u8,Term> {
        ConditionBuilder {
            cond: self.cond,
            value,
            term: self.term
        }
    }

    pub fn term(self, term: ValuedTerm) -> ConditionBuilder<ValuedCondition,Value,ValuedTerm> {
        ConditionBuilder {
            cond: self.cond,
            value: self.value,
            term
        }
    }
}

impl ConditionBuilder<ValuedCondition,u8,ValuedTerm> {
    pub fn build(self) -> Condition {
        Condition::Valued {
            cond: self.cond,
            term: self.term,
            level: self.value,
        }
    }
}

impl ConditionBuilder<ValuedCondition,u8,Empty> {
    pub fn build(self) -> Condition {
        Condition::Valued {
            cond: self.cond,
            term: ValuedTerm::default(),
            level: self.value
        }
    }
}
