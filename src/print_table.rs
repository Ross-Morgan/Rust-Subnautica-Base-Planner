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


fn uniform_length_strings<'a>(array: &Vec<&'a str>, target_length: Option<usize>, fill_char: Option<String>, end_char: Option<String>) -> Vec<&'a str> {
    let t_length = target_length.unwrap_or(1 as usize);
    let f_char = fill_char.unwrap_or(" ".to_string());
    let e_char = end_char.unwrap_or("|".to_string());

    return array.into_iter().map(|s| format!("{}{}{}", s, fill_line(f_char, t_length - s.len()), e_char));
}


fn pretty_print(lines: Vec<&str>, end_char: String, padding: usize) {
    let mut new_lines: Vec<String> = lines.to_vec().iter().map(|&s| s.to_string()).collect();

    let mut fill_char: String;
    let longest_line_length: usize = legnth_of_longest(&lines);

    // If <FILL> exists in list
    if lines.iter().any(|&s| s=="<FILL>") {
        fill_char = lines[lines.len() - 1].to_string();        // Set fill_char to last element of array
        new_lines.pop();                                       // Remove last element of array
    } else {
        fill_char = String::new();
    }

    let line: String = fill_line(&fill_char, longest_line_length);

    // while find_in_vec("<FILL>", &lines).unwrap_or(0 as usize) != 0 {

    // }

    loop {
        let index: usize = find_in_vec("<FILL>", &lines).unwrap_or(usize::MAX);

        if index == usize::MAX { break; }

        let _ = std::mem::replace(&mut new_lines[index], line);
    }

    new_lines = uniform_length_strings(&new_lines, longest_line_length + padding, " ".to_string(), end_char);

    // Join new_lines to String and print it
    println!("{}", new_lines.into_iter().collect::<String>());
}


fn main() {
    let messages: Vec<&str> = vec![
        "Subnautica Base Planner CLI",
        "<FILL>",
        "Use command `add {name} {quantity=1} {add_to_existing/ate=true}` to add item",
        "            `set {name} {quantity}`",
        "            `remove {name} {const=false}` to remove item",
        "            `clear` to remove all components",
        "<FILL>",
        "            `compute {item=null}` to print the materials needed for specified or all components",
        "<FILL>",
        "            `list` to show all added components",
        "            `help` to list valid compontent names",
        "<FILL>",
        "-"
    ];

    pretty_print(messages, "|".to_string(), 1);
}