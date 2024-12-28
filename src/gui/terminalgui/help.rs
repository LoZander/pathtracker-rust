use std::io;

use const_format::concatcp;
use super::CLEAR;

use crate::gui::terminalgui::parser::command_strs;


const HELP_HEADER: &str = "Instructions";
const ITEM: &str = " - ";
const HELP_HELP: &str = concatcp!(
    ITEM, 
    command_strs::HELP, 
    " [<command>]: explains a given command. If none is given, gives this overview."
);
const HELP_END_TURN: &str = concatcp!(
    ITEM, 
    command_strs::END_TURN, 
    ": ends the current turn"
);
const HELP_ADD: &str = concatcp!(
    ITEM, 
    command_strs::ADD, 
    ": adds a character.",
);
const HELP_REMOVE: &str = concatcp!(
    ITEM,
    command_strs::REMOVE,
    " <character>: removes a character from the tracker"
);
const HELP_MODIFY: &str = concatcp!(
    ITEM,
    command_strs::MODIFY,
    " <character> [options]: modifies one or more properties of a character.",
);
const HELP_CONDITION: &str = concatcp!(
    ITEM,
    command_strs::CONDITION,
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
pub enum Topic {
    Summary,
    Help,
    EndTurn,
    Add,
    Remove,
    Modify,
    Condition,
}

fn pause() {
    let mut buff = String::new();
    let stdin = io::stdin();

    let _ = stdin.read_line(&mut buff);
}

const HELP_WITH_HELP: &str = concatcp!(
    command_strs::HELP, " [<command>]:\n\
    \n\
    Gives helpful information on commands. If no argument is given, a general\n\
    command guide and summary list of commands is printed. `help` can also\n\
    give a more detailed description of a command by providing the command as\n\
    an argument.\n\
    \n\
    Example: ", command_strs::HELP, " add"
);

const HELP_WITH_END_TURN: &str = concatcp!(
    command_strs::END_TURN, ":\n\
    \n\
    Ends the current turn.
    "
);

const HELP_WITH_ADD: &str = concatcp!(
    command_strs::ADD, " <init> <name> [<options>]:\n\
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

impl Topic {
    pub fn help(self) {
        println!("{CLEAR}");
        match self {
            Self::Summary => println!("{HELP}"),
            Self::Help => println!("{HELP_WITH_HELP}"),
            Self::EndTurn => println!("{HELP_WITH_END_TURN}"),
            Self::Add => println!("{HELP_WITH_ADD}"),
            Self::Remove => todo!(),
            Self::Modify => todo!(),
            Self::Condition => todo!(),
        };

        println!();

        println!("... press Enter to return");

        pause();    
    }
}
