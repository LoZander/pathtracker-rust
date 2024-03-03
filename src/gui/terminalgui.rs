use std::io;
use const_format::concatcp;
use thiserror::Error;
use crate::{character::{Chr, Health}, tracker::{self, Tracker, TrackerResult}};
use super::Gui;

mod parser;

const CLEAR: &str  = "\x1B[2J\n";
const SPACER: &str = "  ";
const LINE: &str   = "-----------------------------------------------";    
const DLINE: &str  = "===============================================";
const TITLE: &str  = "              ~~~~PATHTRACKER~~~~              ";
const COLUMN: &str = concatcp!(
    "   ", "Init", SPACER, "Dex", SPACER, "Name      ", SPACER,
    "HP     ", SPACER, "Condition(s)");
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

impl Gui for TerminalGui {
    type Err = Error;

    fn run(mut t: Tracker) -> super::GuiResult<Self::Err> {
        let mut buff = String::new();
        let stdin = io::stdin();
        let mut error: Option<Self::Err> = None;
        loop {
            println!("{}", PROLOG);
            for chr in t.get_chrs() {
                println!(
                    "{:^3}{:>4}{SPACER}{:>3}{SPACER}{:<10}{SPACER}{:>3}/{:>3}", 
                    if t.get_in_turn() == Some(chr) { ">" } else { "" },
                    chr.init, 
                    chr.dex.map(|x| x.to_string()).unwrap_or("---".to_string()), 
                    chr.name,
                    chr.health.as_ref().map(|x| x.current.to_string()).unwrap_or("---".to_string()),
                    chr.health.as_ref().map(|x| x.max.to_string()).unwrap_or("---".to_string())
                )
            }
            println!("{}", EPILOG);

            if let Some(err) = error.as_ref() {
                println!("Error: {}", err);
                error = None;
            }

            stdin.read_line(&mut buff)?;
            let res: Result<_, Self::Err> = parser::parse_input(std::mem::take(&mut buff)).map_err(Into::into)
                .and_then(|cmd| execute_command(&mut t, cmd).map_err(Into::into));

            if let Err(err) = res {
                error = Some(err)
            }
        }

    }
}

enum Command {
    EndTurn,
    AddChr { name: String, init: i32, player: bool, dex: Option<i32>, health: Option<u32> },
    RmChr { name: String },
    AddCond { name: String, level: u8 , custom: bool},
    ModName { old: String, new: String },
    ModDex { name: String, val: i32 },
    ModMaxHealth { name: String, val: u32 },
}

fn execute_command(t: &mut Tracker, cmd: Command) -> TrackerResult {
    match cmd {
        Command::EndTurn => {t.end_turn(); Ok(())},
        Command::AddChr { name, init, player, dex , health} => {
            let builder = Chr::builder(name, init, player);
            let builder = match dex {
                None => builder,
                Some(dex) => builder.with_dex(dex)
            };
            let builder = match health {
                None => builder,
                Some(health) => builder.with_health(Health::new(health))
            };
            t.add_chr(builder.build())
        },
        Command::RmChr { name } => t.rm_chr(&name),
        Command::AddCond { name, level, custom } => todo!(),
        Command::ModName { old, new } => todo!(),
        Command::ModDex { name, val } => todo!(),
        Command::ModMaxHealth { name, val } => todo!(),
    }
}

