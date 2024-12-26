use anymap2::AnyMap;
use thiserror::Error;

use super::{Command, Help};

mod condition_parser;
use condition_parser as cond_parser;

#[derive(Debug, Error)]
pub enum Error {
    #[error("empty input.")]
    EmptyInput,

    #[error("missing command.")]
    MissingCommand,

    #[error("invalid keyword `{0}`.")]
    InvalidKeyWord(String),

    #[error("invalid number of args `{0}` for `{1}` command.")]
    InvalidNumberOfArgs(usize, String),

    #[error("invalid condition syntax: expected `<cond> [<value>] <term> on <character>` but got `{0}`")]
    InvalidCondSyntax(String),

    #[error("initiative (first arg) is expected to be a number, but provided arg `{arg}` is not.")]
    ParseInt {
        arg: String,
        #[source] source: std::num::ParseIntError,
    },

    #[error(transparent)]
    InvalidExtraArg(#[from] ExtraArgError),

    #[error(transparent)]
    CondParser(#[from] cond_parser::Error),

    #[error("invalid arg `{0}` for help command.")]
    InvalidHelpArg(String)
}

pub type ParseResult = Result<Command, Error>;

pub mod command_strs {
    pub const END_TURN: &str = "n";
    pub const ADD: &str = "add";
    pub const REMOVE: &str = "rm";
    pub const MODIFY: &str = "mod";
    pub const CONDITION: &str = "cond";
    pub const HELP: &str = "help";
}


pub fn parse_input(input: &str) -> ParseResult {
    let sentences: Vec<&str> = input.split('-').map(str::trim).collect();
    let main: &str = sentences[0];
    let opts = &sentences[1..];

    let words: Vec<&str> = main.split_whitespace().collect();
    let keyword = *words.first().ok_or(Error::EmptyInput)?;
    let args = &words[1..];
    match keyword {
        command_strs::END_TURN => Ok(Command::EndTurn),
        command_strs::ADD => match args {
            [init, name @ ..] => {
                let init: i32 = init.parse().map_err(|err| Error::ParseInt { arg: (*init).to_string(), source: err })?;
                let name = unparse(name);

                let mut map = AnyMap::new();

                for opt in opts {
                    parse_extra_arg(&mut map, opt)?;
                }

                Ok(Command::AddChr { 
                    name, 
                    init,  
                    player: map.get::<PlayerArg>().is_some_and(|x| x.0), 
                    health: map.get::<HealthArg>().map(|x| x.0)
                })
            },
            _ => Err(Error::InvalidNumberOfArgs(args.len(), "add".into())) },
        command_strs::REMOVE => {
            let name = args.iter().intersperse(&" ").fold(String::new(), |x, y| x + y);
            Ok(Command::RmChr { name }) }
        command_strs::MODIFY => {
            let name = args.iter().intersperse(&" ").fold(String::new(), |x, y| x + y);

            let mut map = AnyMap::new();

            for opt in opts {
                parse_extra_arg(&mut map, opt)?;
            }

            
            Ok(Command::Mod {
                name,
                new_name: map.get::<NameArg>().map(|x| x.0.clone()),
                init: map.get::<InitArg>().map(|x| x.0),
                player: map.get::<PlayerArg>().map(|x| x.0),
                health: map.get::<HealthArg>().map(|x| x.0),
            })
        }
        command_strs::CONDITION => {
            let command = cond_parser::parse(args)?;
            Ok(command)
        },
        command_strs::HELP => match args {
            [command_strs::HELP, ..] => Ok(Command::HelpWith(Help::Help)),
            [command_strs::END_TURN, ..] => Ok(Command::HelpWith(Help::EndTurn)),
            [command_strs::ADD, ..] => Ok(Command::HelpWith(Help::Add)),
            [command_strs::REMOVE, ..] => Ok(Command::HelpWith(Help::Remove)),
            [command_strs::MODIFY, ..] => Ok(Command::HelpWith(Help::Modify)),
            [command_strs::CONDITION, ..] => Ok(Command::HelpWith(Help::Condition)),
            [] => Ok(Command::Help),
            other => Err(Error::InvalidHelpArg(unparse(other))),
        },

        word => Err(Error::InvalidKeyWord(word.to_string()))
    }
}

fn unparse(name: &[&str]) -> String {
    name.iter().intersperse(&" ").fold(String::new(), |acc, x| acc + x)
}

struct HealthArg(u32);
struct NameArg(String);
struct InitArg(i32);
struct PlayerArg(bool);

#[derive(Debug, Error)]
pub enum ExtraArgError {
    #[error("extra argument `{typ}` expected an integer but was given `{val}`")]
    ParseIntError {
        typ: String,
        val: String,
        #[source]
        source: std::num::ParseIntError
    },
    #[error("extra argument `{typ}` expected a boolean but was given `{val}`")]
    ParseBoolError {
        typ: String,
        val: String,
        #[source]
        source: std::str::ParseBoolError
    },
}

type ExtraArgResult = Result<(), ExtraArgError>;

fn parse_extra_arg(map: &mut AnyMap, opt: &&str) -> ExtraArgResult {
    let words: Vec<&str> = opt.split_whitespace().collect();
    match &words[..] {
        ["h" | "health", x] => {
            let x: u32 = x.parse().map_err(|err| ExtraArgError::ParseIntError { typ: "-h/-health".into(), val: (*x).to_string(), source: err })?;
            map.insert(HealthArg(x));
        },
        ["n" | "name", x] => {
            map.insert(NameArg((*x).to_string()));
        },
        ["i" | "init", x] => {
            let x: i32 = x.parse().map_err(|err| ExtraArgError::ParseIntError { typ: "-i/-init".into(), val: (*x).to_string(), source: err })?;
            map.insert(InitArg(x));
        },
        ["p" | "player", x] => {
            let x: bool = x.parse().map_err(|err| ExtraArgError::ParseBoolError { typ: "-p/-player".into(), val: (*x).to_string(), source: err })?;
            map.insert(PlayerArg(x));
        },
        ["p" | "player"] => {
            map.insert(PlayerArg(true));
        },
        ["e" | "enemy", x] => {
            let x: bool = x.parse().map_err(|err| ExtraArgError::ParseBoolError { typ: "-e/-enemy".into(), val: (*x).to_string(), source: err })?;
            map.insert(PlayerArg(!x));
        },
        ["e" | "enemy"] => {
            map.insert(PlayerArg(false));
        }
        _ => ()
    }

    Ok(())
}
