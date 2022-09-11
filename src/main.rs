const TITLE: &'static str = "Subnautica Base Planner CLI";

const ENTRY_LINES: [&str; 8] = [
    "----------------------------------------------------------------------------------",
    TITLE,
    "----------------------------------------------------------------------------------",
    "Use command `add {{name}} {{quantity=1}} {{add_to_existing/ate=True}}` to add item",
    "            `set {{name}} {{quantity}}`",
    "            `remove {{name}}` to remove item",
    "            `help` to list valid names",
    "----------------------------------------------------------------------------------",
];


trait Extend {
    fn extend_lines(&self, extend_str: &str) -> Vec<String>;
}


trait Justify {
    fn justify_lines(&self) -> Vec<String>;
}


trait Wrap {
    fn wrap_lines(&self, wrap_str: &str) -> Vec<String>;
}


impl Extend for Vec<String> {
    fn extend_lines(&self, extend_str: &str) -> Vec<String> {
        self.iter().map(|line| format!("{line}{extend_str}")).collect::<Vec<String>>()
    }
}


impl Justify for Vec<String> {
    fn justify_lines(&self) -> Vec<String> {
        // 1. Convert Vec to Iterator
        // 2. Take lengths of strings
        // 3. In case of error return 0
        // 4. Get maximum length

        let max_len: usize = self
            .iter()
            .map(|v: &String| v.len())
            .max()
            .unwrap_or(0);

        // 1. Convert Vec to Iterator
        // 2. Append line with justification string

        if max_len == 0 {
            return self.to_owned();
        }

        self.into_iter().map(
            |line|
            format!("{line}{}", fill_line(" ", max_len - line.len()))
        ).collect::<Vec<String>>()
    }
}


impl Wrap for Vec<String> {
    fn wrap_lines(&self, wrap_str: &str) -> Vec<String> {
        self.iter().map(|line| format!("{wrap_str}{line}{wrap_str}")).collect::<Vec<String>>()
    }
}



fn fill_line(fill_char: &str, length: usize) -> String {
    return (0..length).map(|_| fill_char).collect::<String>();
}


fn pretty_print(lines: Vec<&str>) {
    let lines: Vec<String> = lines
        .into_iter()
        .map(|s: &str| s.to_owned())
        .collect::<Vec<String>>();

    let new_lines: Vec<String> = lines
        .justify_lines()
        .wrap_lines("  ")
        .wrap_lines("|")
        .extend_lines("\n");

    println!("{}", new_lines
        .into_iter()
        .collect::<String>()
    );
}


fn main() {
    pretty_print(ENTRY_LINES.to_vec());
}
