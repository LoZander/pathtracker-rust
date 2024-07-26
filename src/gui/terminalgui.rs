use std::io;
use const_format::concatcp;
use thiserror::Error;
use crate::{character::{Chr, Health}, conditions::Condition, saver::Saver, tracker::{self, Tracker}};

mod parser;

const CLEAR: &str  = "\x1B[2J\n";
const SPACER: &str = "  ";
const LINE: &str   = "-------------------------------------------------------";    
const DLINE: &str  = "=======================================================";
const TITLE: &str  = "| VΛVΛVΛV    <>~<>~<>~PATHTRACKER~<>~<>~<>    VΛVΛVΛV |";
const COLUMN: &str = concatcp!(
    "   ", "Init", SPACER, "P", SPACER, "Name      ", 
    SPACER, "HP     ", SPACER, "Condition(s)");
const HEADER: &str = concatcp!(
    DLINE, "\n",
    TITLE, "\n",
    DLINE, "\n",
    COLUMN, "\n",
    LINE);
const PROLOG: &str = concatcp!(CLEAR, HEADER);
const EPILOG: &str = DLINE;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] parser::Error),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    TrackerError(#[from] tracker::Error)
}

pub struct TerminalGui;

pub fn run<S: Saver>(mut t: Tracker<S>) -> Result<(), Error> {
    let mut buff = String::new();
    let stdin = io::stdin();
    let mut error: Option<Error> = None;
    loop {
        println!("{PROLOG}");
        for chr in t.get_chrs() {
            let mut conds: Vec<String> = t.get_conditions(&chr.name).into_iter()
                .map(ToString::to_string)
                .collect();
            conds.sort();
            let conds_string = conds
                .into_iter()
                .intersperse(format!("\n{:^38}", ""))
                .fold(String::new(), |acc, cond| acc + &cond);
            println!(
                "{:^3}{:>4}{SPACER}{:^1}{SPACER}{:<10}{SPACER}{:>3}/{:>3}{SPACER}{}", 
                if t.get_in_turn() == Some(chr) { ">" } else { "" },
                chr.init, 
                if chr.player {"*"} else {""},
                chr.name,
                chr.health.as_ref().map_or("---".to_string(), |x| x.current.to_string()),
                chr.health.as_ref().map_or("---".to_string(), |x| x.max.to_string()),
                conds_string
            );
        }
        println!("{EPILOG}");

        if let Some(err) = error.as_ref() {
            println!("Error: {err}");
            error = None;
        }

        stdin.read_line(&mut buff)?;
        let res: Result<_, Error> = parser::parse_input(&std::mem::take(&mut buff)).map_err(Into::into)
            .and_then(|cmd| execute_command(&mut t, cmd).map_err(Into::into));

        if let Err(err) = res {
            error = Some(err);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    EndTurn,
    AddChr { 
        name: String, 
        init: i32, 
        player: bool, 
        health: Option<u32> 
    },
    RmChr { name: String },
    AddCond { character: String, cond: Condition },
    Mod { 
        name: String, 
        new_name: Option<String>,
        init: Option<i32>, 
        player: Option<bool>,
        health: Option<u32>
    },
    RmCond { character: String, cond: Condition },
}

fn execute_command<S: Saver>(t: &mut Tracker<S>, cmd: Command) -> tracker::Result<()> {
    match cmd {
        Command::EndTurn => t.end_turn().map(|_| ()),
        Command::AddChr { name, init, player, health} => {
            let builder = Chr::builder(name, init, player);
            let builder = match health {
                None => builder,
                Some(health) => builder.with_health(Health::new(health))
            };
            t.add_chr(builder.build())
        },
        Command::RmChr { name } => t.rm_chr(&name),
        Command::AddCond { character, cond } => t.add_condition(&character, cond),
        Command::RmCond { character, cond } => { t.rm_condition(&character, &cond); Ok(()) },
        Command::Mod { name, new_name, init, player, health } => {
            if let Some(init) = init {
                t.change_init(&name, init)?;
            }

            if let Some(player) = player {
                t.set_player(&name, player)?;
            }

            if let Some(health) = health {
                t.change_max_health(&name, health)?;
            }

            if let Some(new_name) = new_name {
                t.rename(&name, new_name)?;
            }

            Ok(())
        },
    }
}

