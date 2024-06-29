
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
                    let value: u8 = value.parse().map_err(|err| Error::ParseInt { arg: value.to_string(), source: err })?;
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
                    actual: args.iter().intersperse(&" ").fold(String::new(), |acc,word| acc + word).to_string()
                })
            }
        },
        Some(&"rm") => todo!(),
        _ => todo!()
    }
}

pub fn parse_nonvalued_cond(cond_name: &str) -> Result<NonValuedCondition> {
    match cond_name {
        "blinded" => Ok(NonValuedCondition::Blinded),
        "broken" => Ok(NonValuedCondition::Broken),
        "cancealed" => Ok(NonValuedCondition::Concealed),
        "confused" => Ok(NonValuedCondition::Confused),
        "dazzled" => Ok(NonValuedCondition::Dazzled),
        "deafened" => Ok(NonValuedCondition::Deafened),
        "encumbered" => Ok(NonValuedCondition::Encumbered),
        "fascinated" => Ok(NonValuedCondition::Fascinated),
        "fatigued"=> Ok(NonValuedCondition::Fatigued),
        "flat-footed" => Ok(NonValuedCondition::FlatFooted),
        "fleeing" => Ok(NonValuedCondition::Fleeing),
        "friendly" => Ok(NonValuedCondition::Friendly),
        "grabbed" => Ok(NonValuedCondition::Grabbed),
        "helpful" => Ok(NonValuedCondition::Helpful),
        "hidden" => Ok(NonValuedCondition::Hidden),
        "hostile" => Ok(NonValuedCondition::Hostile),
        "immobilized" => Ok(NonValuedCondition::Immobilized),
        "indifferent" => Ok(NonValuedCondition::Indifferent),
        "invisible" => Ok(NonValuedCondition::Invisible),
        "observed" => Ok(NonValuedCondition::Observed),
        "paralyzed" => Ok(NonValuedCondition::Paralyzed),
        "petrified" => Ok(NonValuedCondition::Petrified),
        "prone" => Ok(NonValuedCondition::Prone),
        "quickened" => Ok(NonValuedCondition::Quickened),
        "restrained" => Ok(NonValuedCondition::Restrained),
        "unconscious" => Ok(NonValuedCondition::Unconscious),
        "undetected" => Ok(NonValuedCondition::Undetected),
        "unfriendly" => Ok(NonValuedCondition::Unfriendly),
        "unnoticed" => Ok(NonValuedCondition::Unnoticed),
        s => Err(Error::UndefinedNonValuedCond(s.to_string()))
    }
}

pub fn parse_valued_cond(cond_name: &str) -> Result<ValuedCondition> {
    match cond_name {
        "clumsy" => Ok(ValuedCondition::Clumsy),
        "doomed" => Ok(ValuedCondition::Doomed),
        "drained" => Ok(ValuedCondition::Drained),
        "dying" => Ok(ValuedCondition::Dying),
        "enfeebled" => Ok(ValuedCondition::Enfeebled),
        "frightened" => Ok(ValuedCondition::Frightened),
        "sickened" => Ok(ValuedCondition::Sickened),
        "slowed" => Ok(ValuedCondition::Slowed),
        "stunned" => Ok(ValuedCondition::Stunned),
        "stupified" => Ok(ValuedCondition::Stupified),
        "wounded" => Ok(ValuedCondition::Wounded),
        "persistent:bleed" => Ok(ValuedCondition::PersistentDamage(DamageType::Bleed)),
        "persistent:poison" => Ok(ValuedCondition::PersistentDamage(DamageType::Poison)),
        "persistent:piercing" => Ok(ValuedCondition::PersistentDamage(DamageType::Piercing)),
        "persistent:bludgeoning" => Ok(ValuedCondition::PersistentDamage(DamageType::Bludgeoning)),
        "persistent:slashing" => Ok(ValuedCondition::PersistentDamage(DamageType::Slashing)),
        "persistent:acid" => Ok(ValuedCondition::PersistentDamage(DamageType::Acid)),
        "persistent:cold" => Ok(ValuedCondition::PersistentDamage(DamageType::Cold)),
        "persistent:electricity" => Ok(ValuedCondition::PersistentDamage(DamageType::Electricity)),
        "persistent:sonic" => Ok(ValuedCondition::PersistentDamage(DamageType::Sonic)),
        "persistent:positive" => Ok(ValuedCondition::PersistentDamage(DamageType::Positive)),
        "persistent:negative" => Ok(ValuedCondition::PersistentDamage(DamageType::Negative)),
        "persistent:force" => Ok(ValuedCondition::PersistentDamage(DamageType::Force)),
        "persistent:chaotic" => Ok(ValuedCondition::PersistentDamage(DamageType::Chaotic)),
        "persistent:evil" => Ok(ValuedCondition::PersistentDamage(DamageType::Evil)),
        "persistent:good" => Ok(ValuedCondition::PersistentDamage(DamageType::Good)),
        "persistent:lawful" => Ok(ValuedCondition::PersistentDamage(DamageType::Lawful)),
        s => Err(Error::UndefinedValuedCond(s.to_string()))
    }
}

pub fn parse_nonvalued_term(term_type: &str, term_action: &[&str]) -> Result<NonValuedTerm> {
    match term_type {
        "for" => match term_action {
            [n, unit] => Ok(NonValuedTerm::For(parse_duration(n, unit)?)),
            _ => todo!()
        }
        "until" => parse_turn_event(term_action).map(NonValuedTerm::Until),
        _ => todo!()
    }
}

pub fn parse_valued_term(term_type: &str, term_action: &[&str]) -> Result<ValuedTerm> {
    match term_type {
        "for" => match term_action {
            [n, unit] => Ok(ValuedTerm::For(parse_duration(n, unit)?)),
            _ => todo!()
        } 
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
    let n: Result<u8> = n.parse().map_err(|err| Error::ParseInt { 
        arg: n.to_string(), 
        source: err
    });

    n
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

fn parse_duration(n: &str, unit: &str) -> Result<Duration> {
    let number = n.parse().map_err(|err| Error::ParseInt { 
        arg: n.to_string(), 
        source: err
    })?;
    let dur = match unit {
        "t" | "turn" | "turns" => Duration::from_turns(number),
        "m" | "min" | "mins" | "minute" | "minutes" => 
            Duration::from_minutes(number),
        "h" | "hour" | "hours" => Duration::from_hours(number),
        _ => todo!()
    };

    Ok(dur)
}

#[cfg(test)]
mod tests {
    use crate::{conditions::{Condition, DamageType, NonValuedCondition, NonValuedTerm, TurnEvent, ValuedCondition, ValuedTerm}, gui::terminalgui::Command};

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
}
