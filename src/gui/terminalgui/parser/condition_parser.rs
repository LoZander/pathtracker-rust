use thiserror::Error;
use crate::{conditions::{Condition, DamageType, NonValuedCondition, NonValuedTerm, TurnEvent, ValuedCondition, ValuedTerm}, duration::Duration, gui::terminalgui::Command};

use super::reconstruct_name;


#[derive(Error)]
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    #[error("invalid syntax: expected `{expected}`, actual `{actual}`")]
    InvalidSyntax {
        expected: &'static str,
        actual: String
    },
    #[error("expected one of `add, rm, mod` but got `{0}`")]
    InvalidKeyword(String),
    #[error("undefined nonvalued condition name: `{0}`")]
    UndefinedNonValuedCond(String),
    #[error("undefined valued condition name: `{0}`")]
    UndefinedValuedCond(String),

    #[error("expected condition `{arg}` to be a number")]
    ParseInt {
        arg: String,
        #[source] source: std::num::ParseIntError,
    },
}

type Result<T> = std::result::Result<T,Error>;

pub fn parse(args: &[&str]) -> Result<Command> {
    match args.first() {
        Some(&"add") => {
            let split: Vec<_> = args.split(|s| s == &"on").collect();
            let cond_args = split.first().unwrap();
            let character = reconstruct_name(split.get(1).unwrap());

            match &cond_args.get(1..) {
                Some([cond_name, value]) => {
                    let cond_type = parse_valued_cond(cond_name)?;
                    let value = parse_value(value)?;
                    let cond = Condition::builder().condition(cond_type).value(value).build();
                    Ok(Command::AddCond { character, cond })
                },
                Some([cond_name, value, term_type @ ("for" | "until" | "reduced"), term_trigger @ ..]) => {
                    let value = parse_value(value)?;
                    let cond_type = parse_valued_cond(cond_name)?;
                    let term = parse_valued_term(term_type, term_trigger)?;
                    let cond = Condition::builder()
                        .condition(cond_type)
                        .value(value)
                        .term(term)
                        .build();

                    Ok(Command::AddCond { character, cond })
                }
                Some([cond_name]) => {
                    let cond_type = parse_nonvalued_cond(cond_name)?;
                    let cond = Condition::builder().condition(cond_type).build();
                    Ok(Command::AddCond { character, cond })
                },
                Some([cond_name, term_type @ ("for" | "until"), term_trigger @ ..]) => {
                    let cond_type = parse_nonvalued_cond(cond_name)?;
                    let term = parse_nonvalued_term(term_type, term_trigger)?;
                    let cond = Condition::builder()
                        .condition(cond_type)
                        .term(term)
                        .build();

                    Ok(Command::AddCond{ character, cond })
                }
                _ => Err(Error::InvalidSyntax{
                    expected: "cond add <condition> [<value>] [<termination>] on <character>",
                    actual: args.iter().intersperse(&" ").fold(String::from("add "), |acc,word| acc + word).to_string()
                })
            }
        },
        Some(&"rm") => todo!(),
        _ => todo!()
    }
}

mod nonvalued_conds {
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
}

#[coverage(off)]
pub fn parse_nonvalued_cond(cond_name: &str) -> Result<NonValuedCondition> {
    use nonvalued_conds as conds;
    match cond_name {
        conds::BLINDED     => Ok(NonValuedCondition::Blinded),
        conds::BROKEN      => Ok(NonValuedCondition::Broken),
        conds::CONCEALED   => Ok(NonValuedCondition::Concealed),
        conds::CONFUSED    => Ok(NonValuedCondition::Confused),
        conds::DAZZLED     => Ok(NonValuedCondition::Dazzled),
        conds::DEAFENED    => Ok(NonValuedCondition::Deafened),
        conds::ENCUMBERED  => Ok(NonValuedCondition::Encumbered),
        conds::FASCINATED  => Ok(NonValuedCondition::Fascinated),
        conds::FATIGUED    => Ok(NonValuedCondition::Fatigued),
        conds::FLATFOOTED  => Ok(NonValuedCondition::FlatFooted),
        conds::FLEEING     => Ok(NonValuedCondition::Fleeing),
        conds::FRIENDLY    => Ok(NonValuedCondition::Friendly),
        conds::GRABBED     => Ok(NonValuedCondition::Grabbed),
        conds::HELPFUL     => Ok(NonValuedCondition::Helpful),
        conds::HIDDEN      => Ok(NonValuedCondition::Hidden),
        conds::HOSTILE     => Ok(NonValuedCondition::Hostile),
        conds::IMMOBILIZED => Ok(NonValuedCondition::Immobilized),
        conds::INDIFFERENT => Ok(NonValuedCondition::Indifferent),
        conds::INVISIBLE   => Ok(NonValuedCondition::Invisible),
        conds::OBSERVED    => Ok(NonValuedCondition::Observed),
        conds::PARALYZED   => Ok(NonValuedCondition::Paralyzed),
        conds::PETRIFIED   => Ok(NonValuedCondition::Petrified),
        conds::PRONE       => Ok(NonValuedCondition::Prone),
        conds::QUICKENED   => Ok(NonValuedCondition::Quickened),
        conds::RESTRAINED  => Ok(NonValuedCondition::Restrained),
        conds::UNCONSCIOUS => Ok(NonValuedCondition::Unconscious),
        conds::UNDETECTED  => Ok(NonValuedCondition::Undetected),
        conds::UNFRIENDLY  => Ok(NonValuedCondition::Unfriendly),
        conds::UNNOTICED   => Ok(NonValuedCondition::Unnoticed),
        s => Err(Error::UndefinedNonValuedCond(s.to_string()))
    }
}

mod valued_conds {
    use const_format::concatcp;

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
}

#[coverage(off)]
pub fn parse_valued_cond(cond_name: &str) -> Result<ValuedCondition> {
    use valued_conds as conds;
    match cond_name {
        conds::CLUMSY                 => Ok(ValuedCondition::Clumsy),
        conds::DOOMED                 => Ok(ValuedCondition::Doomed),
        conds::DRAINED                => Ok(ValuedCondition::Drained),
        conds::DYING                  => Ok(ValuedCondition::Dying),
        conds::ENFEEBLED              => Ok(ValuedCondition::Enfeebled),
        conds::FRIGHTENED             => Ok(ValuedCondition::Frightened),
        conds::SICKENED               => Ok(ValuedCondition::Sickened),
        conds::SLOWED                 => Ok(ValuedCondition::Slowed),
        conds::STUNNED                => Ok(ValuedCondition::Stunned),
        conds::STUPIFIED              => Ok(ValuedCondition::Stupified),
        conds::WOUNDED                => Ok(ValuedCondition::Wounded),
        conds::PERSISTENT_BLEED       => Ok(ValuedCondition::PersistentDamage(DamageType::Bleed)),
        conds::PERSISTENT_POISON      => Ok(ValuedCondition::PersistentDamage(DamageType::Poison)),
        conds::PERSISTENT_PIERCING    => Ok(ValuedCondition::PersistentDamage(DamageType::Piercing)),
        conds::PERSISTENT_BLUDGEONING => Ok(ValuedCondition::PersistentDamage(DamageType::Bludgeoning)),
        conds::PERSISTENT_SLASHING    => Ok(ValuedCondition::PersistentDamage(DamageType::Slashing)),
        conds::PERSISTENT_ACID        => Ok(ValuedCondition::PersistentDamage(DamageType::Acid)),
        conds::PERSISTENT_COLD        => Ok(ValuedCondition::PersistentDamage(DamageType::Cold)),
        conds::PERSISTENT_ELECTRICITY => Ok(ValuedCondition::PersistentDamage(DamageType::Electricity)),
        conds::PERSISTENT_SONIC       => Ok(ValuedCondition::PersistentDamage(DamageType::Sonic)),
        conds::PERSISTENT_POSITIVE    => Ok(ValuedCondition::PersistentDamage(DamageType::Positive)),
        conds::PERSISTENT_NEGATIVE    => Ok(ValuedCondition::PersistentDamage(DamageType::Negative)),
        conds::PERSISTENT_FORCE       => Ok(ValuedCondition::PersistentDamage(DamageType::Force)),
        conds::PERSISTENT_CHAOTIC     => Ok(ValuedCondition::PersistentDamage(DamageType::Chaotic)),
        conds::PERSISTENT_EVIL        => Ok(ValuedCondition::PersistentDamage(DamageType::Evil)),
        conds::PERSISTENT_GOOD        => Ok(ValuedCondition::PersistentDamage(DamageType::Good)),
        conds::PERSISTENT_LAWFUL      => Ok(ValuedCondition::PersistentDamage(DamageType::Lawful)),
        s => Err(Error::UndefinedValuedCond(s.to_string()))
    }
}

pub fn parse_nonvalued_term(term_type: &str, term_action: &[&str]) -> Result<NonValuedTerm> {
    match term_type {
        "for" => parse_duration(term_action).map(NonValuedTerm::For),
        "until" => parse_turn_event(term_action).map(NonValuedTerm::Until),
        _ => todo!()
    }
}

pub fn parse_valued_term(term_type: &str, term_action: &[&str]) -> Result<ValuedTerm> {
    match term_type {
        "for" => parse_duration(term_action).map(ValuedTerm::For),
        "until" => parse_turn_event(term_action).map(ValuedTerm::Until),
        "reduced" => match term_action {
            ["by", n, turn_event @ ..] => {
                let n = parse_value(n)?;
                let event = parse_turn_event(turn_event)?;

                Ok(ValuedTerm::Reduced(event, n))
            },
            _ => todo!()
        },
        _ => todo!()
    }
}

fn parse_value(n: &str) -> Result<u8> {
    n.parse().map_err(|err| Error::ParseInt { 
        arg: n.to_string(), 
        source: err
    })
}

fn parse_turn_event(term_action: &[&str]) -> Result<TurnEvent> {
    match term_action {
        ["start", "of", "turn"] => todo!(),
        ["end", "of", "turn"] => todo!(),
        ["start", "of", character @ .., "turn"] => {
            let character = reconstruct_name(character);
            Ok(TurnEvent::StartOfTurn(character))
        },
        ["end", "of", character @ .., "turn"] => {
            let character = reconstruct_name(character);
            Ok(TurnEvent::EndOfTurn(character))
        },
        _ => todo!()
    }
    
}

fn parse_duration(term_action: &[&str]) -> Result<Duration> {
    match term_action {
        [n, unit] => {
            let number = n.parse().map_err(|err| Error::ParseInt { 
                arg: n.to_string(), 
                source: err
            })?;
            let dur = match *unit {
                "t" | "turn" | "turns" => Duration::from_turns(number),
                "m" | "min" | "mins" | "minute" | "minutes" => 
                    Duration::from_minutes(number),
                "h" | "hour" | "hours" => Duration::from_hours(number),
                _ => todo!()
            };

            Ok(dur)
        },
        _ => todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::duration::Duration;
    use crate::gui::terminalgui::Command;
    use crate::conditions::{Condition, DamageType, NonValuedCondition, NonValuedTerm, TurnEvent, ValuedCondition, ValuedTerm};
    use super::Error;

    use super::parse;

    #[test]
    fn add_blinded_on_alice_parses_correctly() {
        let input = ["add","blinded","on","Alice"];
        let command = parse(&input).unwrap();
        let expected = Command::AddCond {
            character: String::from("Alice"),
            cond: Condition::builder()
                .condition(NonValuedCondition::Blinded)
                .build()
        };

        assert_eq!(expected, command)
    }

    #[test]
    fn add_bleed_5_on_alice_parses_correctly() {
        let input = ["add","persistent:bleed","5","on","Bob"];
        let command = parse(&input).unwrap();
        let expected = Command::AddCond {
            character: String::from("Bob"),
            cond: Condition::builder()
                .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
                .value(5)
                .build()
        };

        assert_eq!(expected, command)
    }

    #[test]
    fn add_dazzled_until_end_of_bob_turn_on_alice_parses_correctly() {
        let input = ["add","dazzled","until","end","of","Bob","turn","on","Alice"];
        let command = parse(&input).unwrap();
        let expected = Command::AddCond {
            character: String::from("Alice"),
            cond: Condition::builder()
                .condition(NonValuedCondition::Dazzled)
                .term(NonValuedTerm::Until(TurnEvent::EndOfTurn(String::from("Bob"))))
                .build()
        };

        assert_eq!(expected, command)
    }

    #[test]
    fn add_frightened_2_reduced_by_1_end_of_alice_turn_on_alice_parses_correctly() {
        let input = ["add","frightened","2","reduced","by","1","end","of","Alice","turn","on","Alice"];
        let command = parse(&input).unwrap();
        let expected = Command::AddCond {
            character: String::from("Alice"),
            cond: Condition::builder()
                .condition(ValuedCondition::Frightened)
                .value(2)
                .term(ValuedTerm::Reduced(TurnEvent::EndOfTurn(String::from("Alice")), 1))
                .build()
        };

        assert_eq!(expected, command)
    }

    #[test]
    fn add_drained_2_for_12_hours_on_alice() {
        let input = ["add","drained","2","for","12","hours","on","Alice"];
        let command = parse(&input).unwrap();
        let expected = Command::AddCond {
            character: String::from("Alice"),
            cond: Condition::builder()
                .condition(ValuedCondition::Drained)
                .value(2)
                .term(ValuedTerm::For(Duration::from_hours(12)))
                .build()
        };

        assert_eq!(expected, command)
    }

    #[test]
    fn add_blinded_for_8_hours_on_bob() {
        let input = ["add","blinded","for","8","hours","on","Bob"];
        let command = parse(&input).unwrap();
        let expected = Command::AddCond {
            character: String::from("Bob"),
            cond: Condition::builder()
                .condition(NonValuedCondition::Blinded)
                .term(NonValuedTerm::For(Duration::from_hours(8)))
                .build()
        };

        assert_eq!(expected, command)
    }

    #[test]
    fn add_frightened_negative_2_on_bob() {
        let input = ["add","frightened","-2","on","Bob"];
        let result = parse(&input);
        assert!(matches!(result, Err(Error::ParseInt { .. })))
    }

    mod duration {
        use crate::{duration::Duration, gui::terminalgui::parser::condition_parser::parse_duration};

        #[test]
        fn five_turns_parses() {
            assert_eq!(Ok(Duration::from_turns(5)), parse_duration(&["5","turns"]))
        }

        #[test]
        fn one_turn_parses() {
            assert_eq!(Ok(Duration::from_turns(1)), parse_duration(&["1","turn"]))
        }

        #[test]
        fn two_t_parses() {
            assert_eq!(Ok(Duration::from_turns(2)), parse_duration(&["2","t"]))
        }

        #[test]
        fn three_minutes_parses() {
            assert_eq!(Ok(Duration::from_minutes(3)), parse_duration(&["3","minutes"]))
        }

        #[test]
        fn one_min_parses() {
            assert_eq!(Ok(Duration::from_minutes(1)), parse_duration(&["1","min"]))
        }

        #[test]
        fn one_minute_parses() {
            assert_eq!(Ok(Duration::from_minutes(1)), parse_duration(&["1","minute"]))
        }

        #[test]
        fn ten_m_parses() {
            assert_eq!(Ok(Duration::from_minutes(10)), parse_duration(&["10","m"]))
        }

        #[test]
        fn twelve_hours_parses() {
            assert_eq!(Ok(Duration::from_hours(12)), parse_duration(&["12","hours"]))
        }

        #[test]
        fn one_hour_parses() {
            assert_eq!(Ok(Duration::from_hours(1)), parse_duration(&["1","hour"]))
        }

        #[test]
        fn five_h_parses() {
            assert_eq!(Ok(Duration::from_hours(5)), parse_duration(&["5","h"]))
        }
    }

}
