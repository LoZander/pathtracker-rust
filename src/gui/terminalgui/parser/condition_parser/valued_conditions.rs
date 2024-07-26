use const_format::concatcp;

use crate::conditions::{DamageType, ValuedCondition};

use super::{Error, Result};

pub const CLUMSY: &str         = "clumsy";
pub const DOOMED: &str         = "doomed";
pub const DRAINED: &str        = "drained";
pub const DYING: &str          = "dying";
pub const ENFEEBLED: &str      = "enfeebled";
pub const FRIGHTENED: &str     = "frightened";
pub const SICKENED: &str       = "sickened";
pub const SLOWED: &str         = "slowed";
pub const STUNNED: &str        = "stunned";
pub const STUPIFIED: &str      = "stupified";
pub const WOUNDED: &str        = "wounded";

pub const PERSISTENT: &str     = "persistent";
pub const SEP: &str            = ":";

pub const BLEED: &str          = "bleed";
pub const POISON: &str         = "poison";
pub const PIERCING: &str       = "piercing";
pub const BLUDGEONING: &str    = "bludgeoning";
pub const SLASHING: &str       = "slashing";
pub const ACID: &str           = "acid";
pub const COLD: &str           = "cold";
pub const ELECTRICITY: &str    = "electricity";
pub const SONIC: &str          = "sonic";
pub const POSITIVE: &str       = "positive";
pub const NEGATIVE: &str       = "negative";
pub const FORCE: &str          = "force";
pub const CHAOTIC: &str        = "chaotic";
pub const EVIL: &str           = "evil";
pub const GOOD: &str           = "good";
pub const LAWFUL: &str         = "lawful";

pub const PERSISTENT_BLEED: &str         = concatcp!(PERSISTENT, SEP, BLEED);
pub const PERSISTENT_POISON: &str        = concatcp!(PERSISTENT, SEP, POISON);
pub const PERSISTENT_PIERCING: &str      = concatcp!(PERSISTENT, SEP, PIERCING);
pub const PERSISTENT_BLUDGEONING: &str   = concatcp!(PERSISTENT, SEP, BLUDGEONING);
pub const PERSISTENT_SLASHING: &str      = concatcp!(PERSISTENT, SEP, SLASHING);
pub const PERSISTENT_ACID: &str          = concatcp!(PERSISTENT, SEP, ACID);
pub const PERSISTENT_COLD: &str          = concatcp!(PERSISTENT, SEP, COLD);
pub const PERSISTENT_ELECTRICITY: &str   = concatcp!(PERSISTENT, SEP, ELECTRICITY);
pub const PERSISTENT_SONIC: &str         = concatcp!(PERSISTENT, SEP, SONIC);
pub const PERSISTENT_POSITIVE: &str      = concatcp!(PERSISTENT, SEP, POSITIVE);
pub const PERSISTENT_NEGATIVE: &str      = concatcp!(PERSISTENT, SEP, NEGATIVE);
pub const PERSISTENT_FORCE: &str         = concatcp!(PERSISTENT, SEP, FORCE);
pub const PERSISTENT_CHAOTIC: &str       = concatcp!(PERSISTENT, SEP, CHAOTIC);
pub const PERSISTENT_EVIL: &str          = concatcp!(PERSISTENT, SEP, EVIL);
pub const PERSISTENT_GOOD: &str          = concatcp!(PERSISTENT, SEP, GOOD);
pub const PERSISTENT_LAWFUL: &str        = concatcp!(PERSISTENT, SEP, LAWFUL);

#[coverage(off)]
pub fn parse(cond_name: &str) -> Result<ValuedCondition> {
    match cond_name {
        CLUMSY                 => Ok(ValuedCondition::Clumsy),
        DOOMED                 => Ok(ValuedCondition::Doomed),
        DRAINED                => Ok(ValuedCondition::Drained),
        DYING                  => Ok(ValuedCondition::Dying),
        ENFEEBLED              => Ok(ValuedCondition::Enfeebled),
        FRIGHTENED             => Ok(ValuedCondition::Frightened),
        SICKENED               => Ok(ValuedCondition::Sickened),
        SLOWED                 => Ok(ValuedCondition::Slowed),
        STUNNED                => Ok(ValuedCondition::Stunned),
        STUPIFIED              => Ok(ValuedCondition::Stupified),
        WOUNDED                => Ok(ValuedCondition::Wounded),
        PERSISTENT_BLEED       => Ok(ValuedCondition::PersistentDamage(DamageType::Bleed)),
        PERSISTENT_POISON      => Ok(ValuedCondition::PersistentDamage(DamageType::Poison)),
        PERSISTENT_PIERCING    => Ok(ValuedCondition::PersistentDamage(DamageType::Piercing)),
        PERSISTENT_BLUDGEONING => Ok(ValuedCondition::PersistentDamage(DamageType::Bludgeoning)),
        PERSISTENT_SLASHING    => Ok(ValuedCondition::PersistentDamage(DamageType::Slashing)),
        PERSISTENT_ACID        => Ok(ValuedCondition::PersistentDamage(DamageType::Acid)),
        PERSISTENT_COLD        => Ok(ValuedCondition::PersistentDamage(DamageType::Cold)),
        PERSISTENT_ELECTRICITY => Ok(ValuedCondition::PersistentDamage(DamageType::Electricity)),
        PERSISTENT_SONIC       => Ok(ValuedCondition::PersistentDamage(DamageType::Sonic)),
        PERSISTENT_POSITIVE    => Ok(ValuedCondition::PersistentDamage(DamageType::Positive)),
        PERSISTENT_NEGATIVE    => Ok(ValuedCondition::PersistentDamage(DamageType::Negative)),
        PERSISTENT_FORCE       => Ok(ValuedCondition::PersistentDamage(DamageType::Force)),
        PERSISTENT_CHAOTIC     => Ok(ValuedCondition::PersistentDamage(DamageType::Chaotic)),
        PERSISTENT_EVIL        => Ok(ValuedCondition::PersistentDamage(DamageType::Evil)),
        PERSISTENT_GOOD        => Ok(ValuedCondition::PersistentDamage(DamageType::Good)),
        PERSISTENT_LAWFUL      => Ok(ValuedCondition::PersistentDamage(DamageType::Lawful)),
        s => Err(Error::UndefinedValuedCond(s.to_string()))
    }
}
