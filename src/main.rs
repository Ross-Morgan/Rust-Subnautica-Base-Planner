mod pretty_print;
mod user_input;

use pretty_print::pprint;
use user_input::simple_user_input::get_input;

const TITLE: &'static str = "Subnautica Base Planner CLI";

const ENTRY_LINES: [&str; 12] = [
    "----------------------------------------------------------------------------------",
    TITLE,
    "----------------------------------------------------------------------------------",
    "Use command `add {{name}} {{quantity=1}} {{add_to_existing/ate=True}}` to add item",
    "            `set {{name}} {{quantity}}`",
    "            `remove {{name}}` to remove item",
    "            `help` to list valid names",
    "----------------------------------------------------------------------------------",
    "Use command `save` to leave the program",
    "            `load` to leave the program",
    "            `exit` to leave the program",
    "----------------------------------------------------------------------------------",
];

const INPUT_PROMPT: &'static str = "sub-cli>";


fn main() {
    pprint::pretty_print(ENTRY_LINES.to_vec(), "|".to_string(), 1);

    loop {
        input = get_input(INPUT_PROMPT);
    }

}

