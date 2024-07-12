use thiserror::Error;
use crate::conditions::{Condition, NonValuedTerm, TurnEvent, ValuedTerm};
use crate::duration::Duration;
use super::Command;

use super::reconstruct_name;

mod nonvalued_conditions;
mod valued_conditions;


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
                    let cond_type = valued_conditions::parse(cond_name)?;
                    let value = parse_value(value)?;
                    let cond = Condition::builder().condition(cond_type).value(value).build();
                    Ok(Command::AddCond { character, cond })
                },
                Some([cond_name, value, term_type @ ("for" | "until" | "reduced"), term_trigger @ ..]) => {
                    let value = parse_value(value)?;
                    let cond_type = valued_conditions::parse(cond_name)?;
                    let term = parse_valued_term(term_type, term_trigger)?;
                    let cond = Condition::builder()
                        .condition(cond_type)
                        .value(value)
                        .term(term)
                        .build();

                    Ok(Command::AddCond { character, cond })
                }
                Some([cond_name]) => {
                    let cond_type = nonvalued_conditions::parse(cond_name)?;
                    let cond = Condition::builder().condition(cond_type).build();
                    Ok(Command::AddCond { character, cond })
                },
                Some([cond_name, term_type @ ("for" | "until"), term_trigger @ ..]) => {
                    let cond_type = nonvalued_conditions::parse(cond_name)?;
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

fn parse_nonvalued_term(term_type: &str, term_action: &[&str]) -> Result<NonValuedTerm> {
    match term_type {
        "for" => parse_duration(term_action).map(NonValuedTerm::For),
        "until" => parse_turn_event(term_action).map(NonValuedTerm::Until),
        _ => todo!()
    }
}

fn parse_valued_term(term_type: &str, term_action: &[&str]) -> Result<ValuedTerm> {
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
    use super::{nonvalued_conditions as nv_conds, valued_conditions as v_conds};
    use super::parse;

    #[test]
    fn add_blinded_on_alice_parses_correctly() {
        let input = ["add",nv_conds::BLINDED,"on","Alice"];
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
        let input = ["add",v_conds::PERSISTENT_BLEED,"5","on","Bob"];
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
        let input = ["add",nv_conds::DAZZLED,"until","end","of","Bob","turn","on","Alice"];
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
        let input = ["add",v_conds::FRIGHTENED,"2","reduced","by","1","end","of","Alice","turn","on","Alice"];
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
        let input = ["add",v_conds::DRAINED,"2","for","12","hours","on","Alice"];
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
        let input = ["add",nv_conds::BLINDED,"for","8","hours","on","Bob"];
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
        let input = ["add",v_conds::FRIGHTENED,"-2","on","Bob"];
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
