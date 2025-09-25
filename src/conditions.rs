use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash};

use crate::duration::Duration;

pub mod condition_manager;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialOrd, Ord)]
pub enum Condition {
    Valued { cond: ValuedCondition, term: ValuedTerm, level: u8 },
    NonValued { cond: NonValuedCondition, term: NonValuedTerm }
}

impl Condition {
    #[must_use]
    pub fn builder() -> ConditionBuilder {
        ConditionBuilder::default()
    }

    pub fn to_long_string(&self) -> String {
        match self {
            Self::Valued { cond, term, level } => format!("{cond} {level} {term}"),
            Self::NonValued { cond, term } => format!("{cond} {term}"),
        }
    }

    pub fn to_short_string(&self) -> String {
        match self {
            Condition::Valued { cond, term, level } => format!("{cond} {}", term.to_short_string()),
            Condition::NonValued { cond, term } => format!("{cond} {}", term.to_short_string())
        }
    }
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Valued { cond: c1, .. }, Self::Valued { cond: c2, .. }) => c1 == c2,
            (Self::NonValued { cond: c1, .. }, Self::NonValued { cond: c2, .. }) => c1 == c2,
            _ => false
        }
    }
}

impl Eq for Condition {}

impl Hash for Condition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Valued { cond, .. } => cond.hash(state),
            Self::NonValued { cond, .. } => cond.hash(state),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Valued { cond, term, level } => write!(f, "{cond} {level} {term}"),
            Self::NonValued { cond, term } => write!(f, "{cond} {term}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Hash)]
pub enum TurnEvent {
    StartOfNextTurn(String),
    EndOfNextTurn(String)
}

impl Display for TurnEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartOfNextTurn(name) => write!(f, "start of next turn of {name}"),
            Self::EndOfNextTurn(name) => write!(f, "end of next turn of {name}"),
        }
    }
}


#[derive(Debug, Clone, Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Hash)]
pub enum NonValuedTerm {
    #[default]
    Manual,
    For(Duration),
    Until(TurnEvent)
}

impl NonValuedTerm {
    pub fn to_short_string(&self) -> String {
        match self {
            Self::Manual => String::new(),
            Self::For(_) => "for...".into(),
            Self::Until(_) => "until...".into(),
        }
    }

    pub fn to_long_string(&self) -> String {
        match self {
            Self::Manual => String::new(),
            Self::For(dur) => format!("for {} turns", dur.in_turns()),
            Self::Until(event) => format!("until {event}"),
        }
    }
}

impl Display for NonValuedTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manual => write!(f, ""),
            Self::For(dur) => write!(f, "for {} turns", dur.in_turns()),
            Self::Until(event) => write!(f, "until {event}"),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Hash)]
pub enum ValuedTerm {
    #[default]
    Manual,
    For(Duration),
    Until(TurnEvent),
    Reduced(TurnEvent, u8)
}

impl ValuedTerm {
    pub fn to_short_string(&self) -> String {
        match self {
            ValuedTerm::Manual => String::new(),
            ValuedTerm::For(_) => "for...".into(),
            ValuedTerm::Until(_) => "until...".into(),
            ValuedTerm::Reduced(_, _) => "reduced...".into(),
        }
    }

    pub fn to_long_string(&self) -> String {
        match self {
            Self::Manual => String::new(),
            Self::For(dur) => format!("for {} turns", dur.in_turns()),
            Self::Until(event) => format!("until {event}"),
            Self::Reduced(event, r) => format!("reduced by {r} at {event}"),
        }
    }
}


impl Display for ValuedTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manual => write!(f, ""),
            Self::For(dur) => write!(f, "for {} turns", dur.in_turns()),
            Self::Until(event) => write!(f, "until {event}"),
            Self::Reduced(event, r) => write!(f, "reduced by {r} at {event}"),
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
            Self::PersistentDamage(ty) => write!(f, "p. {ty}"),
            Self::Clumsy => write!(f, "clumsy"),
            Self::Doomed => write!(f, "doomed"),
            Self::Drained => write!(f, "drained"),
            Self::Dying => write!(f, "dying"),
            Self::Enfeebled => write!(f, "enfeebled"),
            Self::Frightened => write!(f, "frightened"),
            Self::Sickened => write!(f, "sickened"),
            Self::Slowed => write!(f, "slowed"),
            Self::Stunned => write!(f, "stunned"),
            Self::Stupified => write!(f, "stupified"),
            Self::Wounded => write!(f, "wounded"),
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
    Encumbered,
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
            Self::Blinded => write!(f, "blinded"),
            Self::Broken => write!(f, "broken"),
            Self::Concealed => write!(f, "concealed"),
            Self::Confused => write!(f, "confused"),
            Self::Controlled => write!(f, "controlled"),
            Self::Dazzled => write!(f, "dazzled"),
            Self::Deafened => write!(f, "deafened"),
            Self::Encumbered => write!(f, "encumbered"),
            Self::Fascinated => write!(f, "fascinated"),
            Self::Fatigued => write!(f, "fatigued"),
            Self::FlatFooted => write!(f, "flat-footed"),
            Self::Fleeing => write!(f, "fleeing"),
            Self::Friendly => write!(f, "friendly"),
            Self::Grabbed => write!(f, "grabbed"),
            Self::Helpful => write!(f, "helpful"),
            Self::Hidden => write!(f, "hidden"),
            Self::Hostile => write!(f, "hostile"),
            Self::Immobilized => write!(f, "immobilized"),
            Self::Indifferent => write!(f, "indifferent"),
            Self::Invisible => write!(f, "invisible"),
            Self::Observed => write!(f, "observed"),
            Self::Paralyzed => write!(f, "paralyzed"),
            Self::Petrified => write!(f, "petrified"),
            Self::Prone => write!(f, "prone"),
            Self::Quickened => write!(f, "quickened"),
            Self::Restrained => write!(f, "restrained"),
            Self::Unconscious => write!(f, "unconscious"),
            Self::Undetected => write!(f, "undetected"),
            Self::Unfriendly => write!(f, "unfriendly"),
            Self::Unnoticed => write!(f, "unnoticed"),
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
            Self::Bleed => write!(f, "bleed"),
            Self::Poison => write!(f, "poison"),
            Self::Piercing => write!(f, "piercing"),
            Self::Bludgeoning => write!(f, "bludgeoning"),
            Self::Slashing => write!(f, "slashing"),
            Self::Acid => write!(f, "acid"),
            Self::Cold => write!(f, "cold"),
            Self::Electricity => write!(f, "electricity"),
            Self::Sonic => write!(f, "sonic"),
            Self::Positive => write!(f, "positive"),
            Self::Negative => write!(f, "negative"),
            Self::Force => write!(f, "force"),
            Self::Chaotic => write!(f, "chaotic"),
            Self::Evil => write!(f, "evil"),
            Self::Good => write!(f, "good"),
            Self::Lawful => write!(f, "lawful"),
        }
    }
}


pub struct Empty;
pub trait CondType {}

impl CondType for ValuedCondition {}
impl CondType for NonValuedCondition {}

pub struct ConditionBuilder<Cond = Empty, Value = Empty, Term = Empty> {
    cond: Cond,
    value: Value,
    term: Term,
}

impl Default for ConditionBuilder<Empty, Empty, Empty> {
    #[must_use]
    fn default() -> Self {
        Self { cond: Empty, value: Empty, term: Empty }
    }
}

impl ConditionBuilder<Empty, Empty, Empty> {
    #[must_use]
    pub const fn condition<Cond: CondType>(self, cond: Cond) -> ConditionBuilder<Cond,Empty,Empty> {
        ConditionBuilder {
            cond,
            value: Empty,
            term: Empty,
        }
    }
}

impl<Term> ConditionBuilder<NonValuedCondition, Empty, Term> {
    #[must_use]
    pub fn term(self, term: NonValuedTerm) -> ConditionBuilder<NonValuedCondition,Empty,NonValuedTerm> {
        ConditionBuilder {
            cond: self.cond,
            value: self.value,
            term,
        }
    }
}

impl ConditionBuilder<NonValuedCondition, Empty, NonValuedTerm> {
    #[must_use]
    pub fn build(self) -> Condition {
        Condition::NonValued {
            cond: self.cond,
            term: self.term,
        }
    }
}

impl ConditionBuilder<NonValuedCondition, Empty, Empty> {
    #[must_use]
    pub fn build(self) -> Condition {
        Condition::NonValued {
            cond: self.cond,
            term: NonValuedTerm::default()
        }
    }
}

impl<Value, Term> ConditionBuilder<ValuedCondition, Value, Term> {
    #[must_use]
    pub fn value(self, value: u8) -> ConditionBuilder<ValuedCondition, u8, Term> {
        ConditionBuilder {
            cond: self.cond,
            value,
            term: self.term
        }
    }

    #[must_use]
    pub fn term(self, term: ValuedTerm) -> ConditionBuilder<ValuedCondition, Value, ValuedTerm> {
        ConditionBuilder {
            cond: self.cond,
            value: self.value,
            term
        }
    }
}

impl ConditionBuilder<ValuedCondition, u8, ValuedTerm> {
    #[must_use]
    pub fn build(self) -> Condition {
        Condition::Valued {
            cond: self.cond,
            term: self.term,
            level: self.value,
        }
    }
}

impl ConditionBuilder<ValuedCondition, u8, Empty> {
    #[must_use]
    pub fn build(self) -> Condition {
        Condition::Valued {
            cond: self.cond,
            term: ValuedTerm::default(),
            level: self.value,
        }
    }
}
