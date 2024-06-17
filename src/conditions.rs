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
