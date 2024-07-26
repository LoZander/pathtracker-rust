use crate::conditions::NonValuedCondition;
use super::{Error, Result};

pub const BLINDED: &str         = "blinded";
pub const BROKEN: &str          = "broken";
pub const CONCEALED: &str       = "concealed";
pub const CONFUSED: &str        = "confused";
pub const DAZZLED: &str         = "dazzled";
pub const DEAFENED: &str        = "deafened";
pub const ENCUMBERED: &str      = "encumbered";
pub const FASCINATED: &str      = "fascinated";
pub const FATIGUED: &str        = "fatigued";
pub const FLATFOOTED: &str      = "flat-footed";
pub const FLEEING: &str         = "fleeing";
pub const FRIENDLY: &str        = "friendly";
pub const GRABBED: &str         = "grabbed";
pub const HELPFUL: &str         = "helpful";
pub const HIDDEN: &str          = "hidden";
pub const HOSTILE: &str         = "hostile";
pub const IMMOBILIZED: &str     = "immobilized";
pub const INDIFFERENT: &str     = "indifferent";
pub const INVISIBLE: &str       = "invisible";
pub const OBSERVED: &str        = "observed";
pub const PARALYZED: &str       = "paralyzed";
pub const PETRIFIED: &str       = "petrified";
pub const PRONE: &str           = "prone";
pub const QUICKENED: &str       = "quickened";
pub const RESTRAINED: &str      = "restrained";
pub const UNCONSCIOUS: &str     = "unconscious";
pub const UNDETECTED: &str      = "undetected";
pub const UNFRIENDLY: &str      = "unfriendly";
pub const UNNOTICED: &str       = "unnoticed";

#[coverage(off)]
pub fn parse(cond_name: &str) -> Result<NonValuedCondition> {
    match cond_name {
        BLINDED     => Ok(NonValuedCondition::Blinded),
        BROKEN      => Ok(NonValuedCondition::Broken),
        CONCEALED   => Ok(NonValuedCondition::Concealed),
        CONFUSED    => Ok(NonValuedCondition::Confused),
        DAZZLED     => Ok(NonValuedCondition::Dazzled),
        DEAFENED    => Ok(NonValuedCondition::Deafened),
        ENCUMBERED  => Ok(NonValuedCondition::Encumbered),
        FASCINATED  => Ok(NonValuedCondition::Fascinated),
        FATIGUED    => Ok(NonValuedCondition::Fatigued),
        FLATFOOTED  => Ok(NonValuedCondition::FlatFooted),
        FLEEING     => Ok(NonValuedCondition::Fleeing),
        FRIENDLY    => Ok(NonValuedCondition::Friendly),
        GRABBED     => Ok(NonValuedCondition::Grabbed),
        HELPFUL     => Ok(NonValuedCondition::Helpful),
        HIDDEN      => Ok(NonValuedCondition::Hidden),
        HOSTILE     => Ok(NonValuedCondition::Hostile),
        IMMOBILIZED => Ok(NonValuedCondition::Immobilized),
        INDIFFERENT => Ok(NonValuedCondition::Indifferent),
        INVISIBLE   => Ok(NonValuedCondition::Invisible),
        OBSERVED    => Ok(NonValuedCondition::Observed),
        PARALYZED   => Ok(NonValuedCondition::Paralyzed),
        PETRIFIED   => Ok(NonValuedCondition::Petrified),
        PRONE       => Ok(NonValuedCondition::Prone),
        QUICKENED   => Ok(NonValuedCondition::Quickened),
        RESTRAINED  => Ok(NonValuedCondition::Restrained),
        UNCONSCIOUS => Ok(NonValuedCondition::Unconscious),
        UNDETECTED  => Ok(NonValuedCondition::Undetected),
        UNFRIENDLY  => Ok(NonValuedCondition::Unfriendly),
        UNNOTICED   => Ok(NonValuedCondition::Unnoticed),
        s => Err(Error::UndefinedNonValuedCond(s.to_string()))
    }
}
