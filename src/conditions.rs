use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash};

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

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Valued { cond, term, level } => write!(f, "{cond} {level} {term}"),
            Condition::NonValued { cond, term } => write!(f, "{cond} {term}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Hash)]
pub enum TurnEvent {
    StartOfTurn(String),
    EndOfTurn(String)
}

impl Display for TurnEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurnEvent::StartOfTurn(name) => write!(f, "start of {name} turn"),
            TurnEvent::EndOfTurn(name) => write!(f, "end of {name} turn"),
        }
    }
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

impl Display for NonValuedTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NonValuedTerm::Manual => write!(f, ""),
            NonValuedTerm::For(dur) => write!(f, "for {} turns", dur.in_turns()),
            NonValuedTerm::Until(event) => write!(f, "until {event}"),
        }
    }
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

impl Display for ValuedTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValuedTerm::Manual => write!(f, ""),
            ValuedTerm::For(dur) => write!(f, "for {} turns", dur.in_turns()),
            ValuedTerm::Until(event) => write!(f, "until {event}"),
            ValuedTerm::Reduced(event, r) => write!(f, "reduced by {r} at {event}"),
        }
    }
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

impl Display for ValuedCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValuedCondition::PersistentDamage(ty) => write!(f, "p. {ty}"),
            ValuedCondition::Clumsy => write!(f, "clumsy"),
            ValuedCondition::Doomed => write!(f, "doomed"),
            ValuedCondition::Drained => write!(f, "drained"),
            ValuedCondition::Dying => write!(f, "dying"),
            ValuedCondition::Enfeebled => write!(f, "enfeebled"),
            ValuedCondition::Frightened => write!(f, "frightened"),
            ValuedCondition::Sickened => write!(f, "sickened"),
            ValuedCondition::Slowed => write!(f, "slowed"),
            ValuedCondition::Stunned => write!(f, "stunned"),
            ValuedCondition::Stupified => write!(f, "stupified"),
            ValuedCondition::Wounded => write!(f, "wounded"),
        }
    }
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

impl Display for NonValuedCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NonValuedCondition::Blinded => write!(f, "blinded"),
            NonValuedCondition::Broken => write!(f, "broken"),
            NonValuedCondition::Concealed => write!(f, "concealed"),
            NonValuedCondition::Confused => write!(f, "confused"),
            NonValuedCondition::Controlled => write!(f, "controlled"),
            NonValuedCondition::Dazzled => write!(f, "dazzled"),
            NonValuedCondition::Deafened => write!(f, "deafened"),
            NonValuedCondition::Encumbered => write!(f, "encumbered"),
            NonValuedCondition::Fascinated => write!(f, "fascinated"),
            NonValuedCondition::Fatigued => write!(f, "fatigued"),
            NonValuedCondition::FlatFooted => write!(f, "flat-footed"),
            NonValuedCondition::Fleeing => write!(f, "fleeing"),
            NonValuedCondition::Friendly => write!(f, "friendly"),
            NonValuedCondition::Grabbed => write!(f, "grabbed"),
            NonValuedCondition::Helpful => write!(f, "helpful"),
            NonValuedCondition::Hidden => write!(f, "hidden"),
            NonValuedCondition::Hostile => write!(f, "hostile"),
            NonValuedCondition::Immobilized => write!(f, "immobilized"),
            NonValuedCondition::Indifferent => write!(f, "indifferent"),
            NonValuedCondition::Invisible => write!(f, "invisible"),
            NonValuedCondition::Observed => write!(f, "observed"),
            NonValuedCondition::Paralyzed => write!(f, "paralyzed"),
            NonValuedCondition::Petrified => write!(f, "petrified"),
            NonValuedCondition::Prone => write!(f, "prone"),
            NonValuedCondition::Quickened => write!(f, "quickened"),
            NonValuedCondition::Restrained => write!(f, "restrained"),
            NonValuedCondition::Unconscious => write!(f, "unconscious"),
            NonValuedCondition::Undetected => write!(f, "undetected"),
            NonValuedCondition::Unfriendly => write!(f, "unfriendly"),
            NonValuedCondition::Unnoticed => write!(f, "unnoticed"),
        }
    }
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

impl Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageType::Bleed => write!(f, "bleed"),
            DamageType::Poison => write!(f, "poison"),
            DamageType::Piercing => write!(f, "piercing"),
            DamageType::Bludgeoning => write!(f, "bludgeoning"),
            DamageType::Slashing => write!(f, "slashing"),
            DamageType::Acid => write!(f, "acid"),
            DamageType::Cold => write!(f, "cold"),
            DamageType::Electricity => write!(f, "electricity"),
            DamageType::Sonic => write!(f, "sonic"),
            DamageType::Positive => write!(f, "positive"),
            DamageType::Negative => write!(f, "negative"),
            DamageType::Force => write!(f, "force"),
            DamageType::Chaotic => write!(f, "chaotic"),
            DamageType::Evil => write!(f, "evil"),
            DamageType::Good => write!(f, "good"),
            DamageType::Lawful => write!(f, "lawful"),
        }
    }
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
