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

const HELP_WITH_REMOVE: &str = concatcp!(
    command_strs::REMOVE, " <name>:\n\
    \n\
    Removes a character from the tracker.\n\
    \n\
    If it's the to-be-removed characters turn then upon removal their turn\n\
    ends as if the ", command_strs::END_TURN, " command had been invoked.\n\
    \n\
    Example: ", command_strs::REMOVE, " Carlile"
);

const HELP_WITH_MODIFY: &str = concatcp!(
    command_strs::MODIFY, " <name> [<options>]:\n\
    \n\
    Modifies an attribute of a character.\n\
    \n\
    The attributes to be modified are specified by giving one or more options.\n\
    The options are:\n\
     - name/n <new name>: changes the name of the character\n\
     - init/i <new init>: changes the initiative of the character\n\
     - health/h <max health>: adds health tracking to character and/or changes max health.\n\
     - player/p: marks the character as a player character\n\
     - enemy/e: marks the character as an enemy character\n\
    \n\
    Example: ", command_strs::MODIFY, " Sarah -h 23 -p"
);

const HELP_WITH_CONDITION: &str = concatcp!(
    command_strs::CONDITION, " <cond command>:\n\
    \n\
    Allows adding and removing conditions via the following condition commands:\n\
     - add <condition> [<condition level>] [<term criteria>] on <character>: adds the given condition to the given character.\n\
     - rm <condition> from <character>: removes the given condition from the given character.\n\
    \n\
    Example: ", command_strs::CONDITION, " add clumsy 2 until end of turn on Clara\n\
    \n\
    Conditions (<condition>):\n\
    \n\
    The tracker supports all of the standard conditions and most have the obvious name.\n\
    For persistent damage we have the special notation 'persistent:<damage type>',\n\
    where the damage are those from the legacy version of PF2E (not remastered).\n\
    \n\
    Termination criteria (<term criteria>):\n\
    \n\
    Specifying a termination criteria allows the tracker to automatically remove or decrement the condition\n\
    on a given trigger. If no termination criteria is given, the condition must be manually managed.\n\
    The tracker supports the following termination criteria:\n\
     - until <trigger>\n\
     - for <time>\n\
     - reduced <trigger>\n\
    \n\
    Triggers (<trigger>) we have:\n\
     - end of [<character>] turn\n\
     - start of [<character>] turn\n\
    \n\
    where not supplying a character name makes the trigger relative to the\n\
    character whose condition it is. For instance, if we add 'slowed 2' to\n\
    'Clara' with the trigger 'until start of turn' then 'Clara' is slowed 2 until\n\
    the start of her turn. If we instead wrote 'until start of Mathias turn' then\n\
    Clara will be slowed 2 until the start of Mathias turn.\n\
    \n\
    Time (<time>):\n\
    \n\
    For time based termination criteria, we can specify time in actions, turns,\n\
    seconds, minutes and even days. Note however, that the tracker doesn't track time\n\
    on a finer granularity than turns, so something that lasts for 2 actions, for instance\n\
    will terminate after a turn. Be aware that we track the time of a condition relative\n\
    to the end of the turn of the character who has the condition.\n\
    For instance, if Clara is blinded for 2 turns then this condition ends one\n\
    Clara's turn has ended twice.\n\
    \n\
    \n\
    Known issues:\n\
    \n\
    A known issue is that if a character gets a condition on their turn that should\n\
    last until the end of their next turn, then using the trigger 'until end of turn'\n\
    maybe somewhat unintuitively will make the condition last until the end of\n\
    the current turn, not the next one. For now this issue can be sidestepped\n\
    by instead using the trigger 'for 1 turn' in this situation."
);

impl Topic {
    pub fn help(self) {
        println!("{CLEAR}");
        match self {
            Self::Summary => println!("{HELP}"),
            Self::Help => println!("{HELP_WITH_HELP}"),
            Self::EndTurn => println!("{HELP_WITH_END_TURN}"),
            Self::Add => println!("{HELP_WITH_ADD}"),
            Self::Remove => println!("{HELP_WITH_REMOVE}"),
            Self::Modify => println!("{HELP_WITH_MODIFY}"),
            Self::Condition => println!("{HELP_WITH_CONDITION}"),
        };

        println!();

        println!("... press Enter to return");

        pause();    
    }
}
