pub fn get_first_author_last_name(authors: &str) -> Result<&str, &'static str> {
    // Note that we use iterators instead of collecting into vectors for performance
    let first_author: &str = authors.split("and").next().ok_or("No author found")?.trim();
    let last_name: &str = first_author
        .split_whitespace()
        .last()
        .ok_or("No last name found")?;
    Ok(last_name)
}

pub fn wrap_metadata_lines(year: &u16, last_name: &str, lines: Vec<String>) -> String {
    // Preallocate memory for the output string
    let capacity: usize =
        50 + last_name.len() + lines.iter().map(|s: &String| s.len()).sum::<usize>();
    let mut output: String = String::with_capacity(capacity);

    // Construct the output string efficiently by appending instead of repeated memory allocations
    output.push_str(&format!("@article{{{}{},\n", last_name, year));
    for val in lines {
        output.push_str(&format!("    {},\n", val));
    }
    output.push_str("}\n");

    output
}

#[cfg(test)]
mod tests_get_first_author_last_name {
    use super::get_first_author_last_name;

    #[test]
    fn test_correct_last_name() {
        let authors: String = String::from(
            "
    Test McTest and Test2 McTest2 and Test3 McTest3
        ",
        );
        let result: &str = get_first_author_last_name(&authors).unwrap();
        let expected: &str = "McTest";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one_author() {
        let authors: String = String::from(
            "
    Test McTest
        ",
        );
        let result: &str = get_first_author_last_name(&authors).unwrap();
        let expected: &str = "McTest";
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests_wrap_metadata_lines {
    use super::wrap_metadata_lines;

    #[test]
    fn test_correct_wrap() {
        let lines: Vec<String> = vec![
            String::from("title = {test}"),
            String::from("author = {Test McTest}"),
        ];
        let result: String = wrap_metadata_lines(&2000, "McTest", lines);
        let expected: String = String::from(
            "@article{McTest2000,
    title = {test},
    author = {Test McTest},
}\n",
        );
        assert_eq!(result, expected);
    }
}
