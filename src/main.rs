mod parse;
mod pprint;

use std::process;

fn main() {
    let entry_lines: Vec<&str> = vec![
        "--------------------------------------------------------",
        "Subnautica Base Planner CLI",
        "--------------------------------------------------------",
        "Use command `add <name> <quantity=1> to add item",
        "            `set <name> <quantity>` to set item quantity",
        "            `remove <name> <quantity=1>` to remove item",
        "            `howmany <name> to show quantity of item",
        "            `list to list all added items`",
        "            `help` to list valid names",
        "--------------------------------------------------------",
    ];

    pprint::pretty_print(entry_lines);

    let exit_code: i32 = parse::run_shell();

    process::exit(exit_code);
}
