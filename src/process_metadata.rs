pub fn get_first_author_last_name(authors: &String) -> &str {
    let authors: Vec<&str> = authors.split("and").collect();
    let first: &str = authors[0].trim();
    let first: Vec<&str> = first.split(" ").collect();
    let last_name: &str = first[first.len() - 1];
    last_name
}

pub fn wrap_metadata_lines(year: &u16, last_name: &str, lines: Vec<String>) -> String {
    let mut output: String = format!("@article{{{}{},\n", last_name, year);
    for val in lines {
        output = format!("{}    {},\n", output, val);
    }
    output = format!("{}}}\n", output);
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
        let result: &str = get_first_author_last_name(&authors);
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
        let result: &str = get_first_author_last_name(&authors);
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
