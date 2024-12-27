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

/// Runs the Tracker using the terminal GUI.
///
/// # Errors
///
/// This function will return an error if
/// - Reading terminal input fails
/// - Parsing terminal input fails
/// - [`Tracker<S>`] fails when executing a command.
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
    Help,
    HelpWith(Help),
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
        Command::Help => {
            help();
            Ok(())
        }
        Command::HelpWith(cmd) => {
            help_with(cmd);
            Ok(())
        }
    }
}

const HELP_HEADER: &str = "Instructions";
const ITEM: &str = " - ";
const INDENT: &str = "   ";
const HELP_HELP: &str = concatcp!(
    ITEM, 
    parser::command_strs::HELP, 
    " [<command>]: explains a given command. If none is given, gives this overview."
);
const HELP_END_TURN: &str = concatcp!(
    ITEM, 
    parser::command_strs::END_TURN, 
    ": ends the current turn"
);
const HELP_ADD: &str = concatcp!(
    ITEM, 
    parser::command_strs::ADD, 
    ": adds a character.",
);
const HELP_REMOVE: &str = concatcp!(
    ITEM,
    parser::command_strs::REMOVE,
    " <character>: removes a character from the tracker"
);
const HELP_MODIFY: &str = concatcp!(
    ITEM,
    parser::command_strs::MODIFY,
    " <character> [options]: modifies one or more properties of a character.",
);
const HELP_CONDITION: &str = concatcp!(
    ITEM,
    parser::command_strs::CONDITION,
    " <condition command>: do `help cond` for details.",
);

const HELP: &str = concatcp!(
    HELP_HEADER, "\n", 
    HELP_HELP, "\n",
    HELP_END_TURN, "\n",
    HELP_ADD, "\n",
    HELP_REMOVE, "\n",
    HELP_MODIFY, "\n",
    HELP_CONDITION
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Help {
    Help,
    EndTurn,
    Add,
    Remove,
    Modify,
    Condition,
}

fn help() {
    println!("{CLEAR}");
    println!("{HELP}");

    let mut buff = String::new();
    let stdin = io::stdin();

    let _ = stdin.read_line(&mut buff);
}

fn pause() {
    let mut buff = String::new();
    let stdin = io::stdin();

    let _ = stdin.read_line(&mut buff);
}


const HELP_WITH_HELP: &str = concatcp!(
    parser::command_strs::HELP, " [<command>]:\n\
    \n\
    Gives helpful information on commands. If no argument is given, a general\n\
    command guide and summary list of commands is printed. `help` can also\n\
    give a more detailed description of a command by providing the command as\n\
    an argument.\n\
    \n\
    Example: ", parser::command_strs::HELP, " add"
);

const HELP_WITH_END_TURN: &str = concatcp!(
    parser::command_strs::END_TURN, ":\n\
    \n\
    Ends the current turn.
    "
);

const HELP_WITH_ADD: &str = concatcp!(
    parser::command_strs::ADD, " <init> <name> [<options>]:\n\
    \n\
    Adds a new character to the tracker.\n\
    \n\
    Optional features like tracking health can be added by including the\n\
    corresponding option. The options are:\n\
     - health/h <max health>: adds health tracking\n\
     - player/p: marks the character as a player character\n\
     - enemy/e: marks the character as an enemy character\n\
    \n\
    Example: add 24 Sarah -player -health 20
    "
);

fn help_with(cmd: Help) {
    println!("{CLEAR}");
    match cmd {
        Help::Help => println!("{HELP_WITH_HELP}"),
        Help::EndTurn => println!("{HELP_WITH_END_TURN}"),
        Help::Add => println!("{HELP_WITH_ADD}"),
        Help::Remove => todo!(),
        Help::Modify => todo!(),
        Help::Condition => todo!(),
    };

    println!();

    println!("... press Enter to return");

    pause();
}
