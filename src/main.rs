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



fn fill_line(fill_char: &str, length: usize) -> String {
    // Create range of length 'n' and replace chars with chosen character
    return (0..length).map(|_| fill_char).collect::<String>();
}


fn find_in_vec(value: &'static str, array: &Vec<&str>) -> Option<usize> {
    // Convert &Vec<&str> to Vec<String>
    let s_array: Vec<String> = array.to_owned().into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    // Return Option<{index of value}>
    return s_array.iter().position(|x| x == value);
}


fn wrap_line(line: &str, wrap_char: &str, pad_char: &str, padding: usize) -> String {
    format!("{}{}{}{}{}", wrap_char, fill_line(pad_char, padding), line, fill_line(" ", padding), wrap_char)
}


fn wrap_lines(array: Vec<String>, wrap_char: &str, padding: usize) -> Vec<String> {
    array.into_iter().map(|s| wrap_line(s.as_str(), wrap_char, " ", padding)).collect::<Vec<String>>()
}


fn legnth_of_longest(array: &Vec<&str>) -> usize {
    if array.len() == 0 {
        return 0;
    }

    assert!(array.len() < usize::MAX);

    return array.iter().fold(array[0], |acc, &item| {
        if item.len() > acc.len() {
            item
        } else {
            acc
        }
    }).len();
}


fn uniform_length_strings(array: Vec<String>, target_length: Option<usize>, fill_char: Option<String>) -> Vec<String> {
    let t_length = target_length.unwrap_or(1 as usize);
    let f_char = fill_char.unwrap_or(" ".to_string());

    return array.into_iter().map(|s| format!("{}{}", s, fill_line(&f_char, t_length - s.len()))).collect::<Vec<String>>();
}


fn extend_lines(array: Vec<String>, s: &str) -> Vec<String> {
    array.into_iter().map(|ele| format!("{}{}", ele, s.to_owned())).collect::<Vec<String>>()
}


fn pretty_print(lines: Vec<&str>, end_char: String, padding: usize) {
    let mut new_lines: Vec<String> = lines.to_vec().iter().map(|&s| s.to_string()).collect();

    let longest_line_length: usize = legnth_of_longest(&lines);

    let fill_char = {
        // If <FILL> exists in list
        let mut c = " ";

        if lines.iter().any(|&s| s=="<FILL>") {
            new_lines.pop();
            c = lines[lines.len() - 1]
        }

        c
    };

    let filling_line = &fill_line(fill_char, longest_line_length);

    loop {
        let index: usize = find_in_vec("<FILL>", &lines).unwrap_or(usize::MAX);

        if index == usize::MAX { break; }

        let _ = std::mem::replace(&mut new_lines[index], filling_line.to_string());
    }

    new_lines = uniform_length_strings(new_lines, Some(longest_line_length), Some(" ".to_string()));
    new_lines = wrap_lines(new_lines, &end_char, padding);
    new_lines = extend_lines(new_lines, "\n");

    // Join new_lines to String and print it
    println!("{}", new_lines.into_iter().collect::<String>());
}


fn main() {
    pretty_print(ENTRY_LINES.to_vec(), "|".to_string(), 1);
}
