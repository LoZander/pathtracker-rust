use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash};

use crate::{character::ChrName, duration::Duration, settings::Pf2eVersion};

pub mod condition_manager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(Serialize, Deserialize)]
pub enum CondDetail {
    #[default]
    Long,
    Short
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(Serialize, Deserialize)]
pub struct CondFormat {
    detail: CondDetail,
    version: Pf2eVersion
}

impl CondFormat {
    #[must_use]
    pub const fn set_version(mut self, v: Pf2eVersion) -> Self {
        self.version = v;
        self
    }

    #[must_use]
    pub const fn get_version(&self) -> Pf2eVersion {
        self.version
    }

    #[must_use]
    pub const fn get_detail(&self) -> CondDetail {
        self.detail
    }
}

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

    #[must_use]
    pub fn to_string(&self, format: CondFormat) -> String {
        match self {
            Self::Valued { cond, term, level } => {
                match format {
                    CondFormat { detail: CondDetail::Short, version: Pf2eVersion::Remastered } => format!("{} {level} {}", cond.to_remas_string(), term.to_short_string()),
                    CondFormat { detail: CondDetail::Long, version: Pf2eVersion::Remastered } => format!("{} {level} {}", cond.to_remas_string(), term.to_long_string()),
                    CondFormat { detail: CondDetail::Short, version: Pf2eVersion::Old } => format!("{} {level} {}", cond.to_old_string(), term.to_short_string()),
                    CondFormat { detail: CondDetail::Long, version: Pf2eVersion::Old } => format!("{} {level} {}", cond.to_old_string(), term.to_long_string()),
                }
            }
            Self::NonValued { cond, term } => {
                match format {
                    CondFormat { detail: CondDetail::Short, version: Pf2eVersion::Remastered } => format!("{} {}", cond.to_remas_string(), term.to_short_string()),
                    CondFormat { detail: CondDetail::Long, version: Pf2eVersion::Remastered } => format!("{} {}", cond.to_remas_string(), term.to_long_string()),
                    CondFormat { detail: CondDetail::Short, version: Pf2eVersion::Old } => format!("{} {}", cond.to_old_string(), term.to_short_string()),
                    CondFormat { detail: CondDetail::Long, version: Pf2eVersion::Old } => format!("{} {}", cond.to_old_string(), term.to_long_string()),
                }
            }
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Hash)]
pub enum TurnEvent {
    StartOfNextTurn(ChrName),
    EndOfNextTurn(ChrName),
    EndOfCurrentTurn(ChrName),
}

impl Display for TurnEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartOfNextTurn(name) => write!(f, "start of next turn of {name}"),
            Self::EndOfNextTurn(name) => write!(f, "end of next turn of {name}"),
            Self::EndOfCurrentTurn(name) => write!(f, "end of current turn of {name}"),
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
    #[must_use]
    pub fn to_short_string(&self) -> String {
        match self {
            Self::Manual => String::new(),
            Self::For(_) => "for...".into(),
            Self::Until(_) => "until...".into(),
        }
    }

    #[must_use]
    pub fn to_long_string(&self) -> String {
        match self {
            Self::Manual => String::new(),
            Self::For(dur) => format!("for {} turns", dur.in_turns()),
            Self::Until(event) => format!("until {event}"),
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
    #[must_use] 
    pub fn to_short_string(&self) -> String {
        match self {
            Self::Manual => String::new(),
            Self::For(_) => "for...".into(),
            Self::Until(_) => "until...".into(),
            Self::Reduced(_, _) => "reduced...".into(),
        }
    }

    #[must_use]
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

impl ValuedCondition {
    #[must_use]
    pub fn to_string(self, format: CondFormat) -> String {
        match format.version {
            Pf2eVersion::Old => self.to_old_string(),
            Pf2eVersion::Remastered => self.to_remas_string(),
        }
    }

    #[must_use]
    pub fn to_old_string(self) -> String {
        match self {
            Self::PersistentDamage(ty) => format!("p. {}", ty.to_old_string()),
            Self::Clumsy => "clumsy".into(),
            Self::Doomed => "doomed".into(),
            Self::Drained => "drained".into(),
            Self::Dying => "dying".into(),
            Self::Enfeebled => "enfeebled".into(),
            Self::Frightened => "frightened".into(),
            Self::Sickened => "sickened".into(),
            Self::Slowed => "slowed".into(),
            Self::Stunned => "stunned".into(),
            Self::Stupified => "stupified".into(),
            Self::Wounded => "wounded".into(),
        }
    }

    #[must_use]
    pub fn to_remas_string(self) -> String {
        match self {
            Self::PersistentDamage(ty) => format!("p. {}", ty.to_remas_string()),
            Self::Clumsy => "clumsy".into(),
            Self::Doomed => "doomed".into(),
            Self::Drained => "drained".into(),
            Self::Dying => "dying".into(),
            Self::Enfeebled => "enfeebled".into(),
            Self::Frightened => "frightened".into(),
            Self::Sickened => "sickened".into(),
            Self::Slowed => "slowed".into(),
            Self::Stunned => "stunned".into(),
            Self::Stupified => "stupified".into(),
            Self::Wounded => "wounded".into(),
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

impl NonValuedCondition {
    #[must_use]
    pub fn to_string(self, format: CondFormat) -> String {
        match format.version {
            Pf2eVersion::Old => self.to_old_string(),
            Pf2eVersion::Remastered => self.to_remas_string(),
        }
    }

    #[must_use]
    fn to_old_string(self) -> String {
        match self {
            Self::Blinded => "blinded",
            Self::Broken => "broken",
            Self::Concealed => "concealed",
            Self::Confused => "confused",
            Self::Controlled => "controlled",
            Self::Dazzled => "dazzled",
            Self::Deafened => "deafened",
            Self::Encumbered => "encumbered",
            Self::Fascinated => "fascinated",
            Self::Fatigued => "fatigued",
            Self::FlatFooted => "flat-footed",
            Self::Fleeing => "fleeing",
            Self::Friendly => "friendly",
            Self::Grabbed => "grabbed",
            Self::Helpful => "helpful",
            Self::Hidden => "hidden",
            Self::Hostile => "hostile",
            Self::Immobilized => "immobilized",
            Self::Indifferent => "indifferent",
            Self::Invisible => "invisible",
            Self::Observed => "observed",
            Self::Paralyzed => "paralyzed",
            Self::Petrified => "petrified",
            Self::Prone => "prone",
            Self::Quickened => "quickened",
            Self::Restrained => "restrained",
            Self::Unconscious => "unconscious",
            Self::Undetected => "undetected",
            Self::Unfriendly => "unfriendly",
            Self::Unnoticed => "unnoticed",
        }.into()
    }

    #[must_use]
    fn to_remas_string(self) -> String {
        match self {
            Self::Blinded => "blinded",
            Self::Broken => "broken",
            Self::Concealed => "concealed",
            Self::Confused => "confused",
            Self::Controlled => "controlled",
            Self::Dazzled => "dazzled",
            Self::Deafened => "deafened",
            Self::Encumbered => "encumbered",
            Self::Fascinated => "fascinated",
            Self::Fatigued => "fatigued",
            Self::FlatFooted => "off-guard",
            Self::Fleeing => "fleeing",
            Self::Friendly => "friendly",
            Self::Grabbed => "grabbed",
            Self::Helpful => "helpful",
            Self::Hidden => "hidden",
            Self::Hostile => "hostile",
            Self::Immobilized => "immobilized",
            Self::Indifferent => "indifferent",
            Self::Invisible => "invisible",
            Self::Observed => "observed",
            Self::Paralyzed => "paralyzed",
            Self::Petrified => "petrified",
            Self::Prone => "prone",
            Self::Quickened => "quickened",
            Self::Restrained => "restrained",
            Self::Unconscious => "unconscious",
            Self::Undetected => "undetected",
            Self::Unfriendly => "unfriendly",
            Self::Unnoticed => "unnoticed",
        }.into()
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

impl DamageType {
    #[must_use]
    pub fn to_old_string(self) -> String {
        match self {
            Self::Bleed => "bleed",
            Self::Poison => "poison",
            Self::Piercing => "piercing",
            Self::Bludgeoning => "bludgeoning",
            Self::Slashing => "slashing",
            Self::Acid => "acid",
            Self::Cold => "cold",
            Self::Electricity => "electricity",
            Self::Sonic => "sonic",
            Self::Positive => "positive",
            Self::Negative => "negative",
            Self::Force => "force",
            Self::Chaotic => "chaotic",
            Self::Evil => "evil",
            Self::Good => "good",
            Self::Lawful => "lawful",
        }.into()
    }

    #[must_use]
    pub fn to_remas_string(self) -> String {
        match self {
            Self::Bleed => "bleed",
            Self::Poison => "poison",
            Self::Piercing => "piercing",
            Self::Bludgeoning => "bludgeoning",
            Self::Slashing => "slashing",
            Self::Acid => "acid",
            Self::Cold => "cold",
            Self::Electricity => "clectricity",
            Self::Sonic => "sonic",
            Self::Positive => "vitality",
            Self::Negative => "void",
            Self::Force => "force",
            Self::Chaotic => "chaotic?",
            Self::Evil => "evil?",
            Self::Good => "good?",
            Self::Lawful => "lawful?",
        }.into()
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
