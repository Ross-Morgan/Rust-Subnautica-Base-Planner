const TITLE: &'static str = "Subnautica Base Planner CLI";

const ENTRY_LINES: [&str; 7] = [
    TITLE,
    "------------------------------------------------------------------------------",
    "Use command `add {{name}} {{quantity=1}} {{add_to_existing/ate=True}}` to add item",
    "            `set {{name}} {{quantity}}`",
    "            `remove {{name}}` to remove item",
    "            `help` to list valid names",
    "------------------------------------------------------------------------------",
];


fn find_in_vec(value: &'static str, array: &Vec<&str>) -> Option<usize> {
    // Convert &Vec<&str> to Vec<String>
    let s_array: Vec<String> = array.to_owned().iter().map(|&s| s.to_string()).collect();

    // Return Option<{index of value}>
    return s_array.iter().position(|x: &String| x == value);
}


fn legnth_of_longest<'a>(array: &Vec<&'a str>) -> usize {
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


fn fill_line(fill_char: &str, length: usize) -> String {
    // Create range of length 'n' and replace chars with chosen character
    return (0..length).map(|_| fill_char).collect::<String>();
}


fn uniform_length_strings(array: &Vec<String>, target_length: Option<usize>, fill_char: Option<String>, end_char: Option<String>) -> Vec<String> {
    let t_length = target_length.unwrap_or(1 as usize);
    let f_char = fill_char.unwrap_or(" ".to_string());
    let mut e_char = end_char.unwrap_or("|".to_string());

    e_char.push_str("\n");

    return array.into_iter().map(|s| format!("{}{}{}", s, fill_line(&f_char, t_length - s.len()), e_char)).collect::<Vec<String>>();
}


fn pretty_print(lines: Vec<&str>, end_char: String, padding: usize) {
    let mut new_lines: Vec<String> = lines.to_vec().iter().map(|&s| s.to_string()).collect();

    let longest_line_length: usize = legnth_of_longest(&lines);

    let fill_char = loop {
        // If <FILL> exists in list
        if lines.iter().any(|&s| s=="<FILL>") {
            let c = lines[lines.len() - 1];
            new_lines.pop();
            break c;
        }
        break " ";
    };

    let line = &fill_line(fill_char, longest_line_length);

    loop {
        let index: usize = find_in_vec("<FILL>", &lines).unwrap_or(usize::MAX);

        if index == usize::MAX { break; }

        let _ = std::mem::replace(&mut new_lines[index], line.to_string());
    }

    new_lines = uniform_length_strings(&new_lines, Some(longest_line_length + padding), Some(" ".to_string()), Some(end_char));

    // Join new_lines to String and print it
    println!("{}", new_lines.into_iter().collect::<String>());
}


fn main() {
    pretty_print(ENTRY_LINES.to_vec(), "|".to_string(), 1);
}
