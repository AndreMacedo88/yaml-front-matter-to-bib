use serde::Deserialize;

#[derive(Deserialize)]
pub struct MetadataBio {
    pub title: String,
    pub author: String,
    pub journal: String,
    pub year: u16,
    pub volume: u32,
    pub number: u32,
    pub pages: String,
}

pub fn generate_bib_metadata_lines<'a>(metadata: &MetadataBio) -> Vec<String> {
    vec![
        format!("title = {{{}}}", metadata.title),
        format!("author = {{{}}}", metadata.author),
        format!("journal = {{{}}}", metadata.journal),
        format!("year = {{{}}}", metadata.year),
        format!("volume = {{{}}}", metadata.volume),
        format!("number = {{{}}}", metadata.number),
        format!("pages = {{{}}}", metadata.pages),
    ]
}

#[cfg(test)]
mod tests_generate_bib_metadata_lines {
    use super::*;

    #[test]
    fn test_correct_lines() {
        let metadata: MetadataBio = MetadataBio {
            title: String::from("1"),
            author: String::from("2"),
            journal: String::from("3"),
            year: 4,
            volume: 5,
            number: 6,
            pages: String::from("7-10"),
        };
        let result: Vec<String> = generate_bib_metadata_lines(&metadata);
        let expected: Vec<String> = vec![
            String::from("title = {1}"),
            String::from("author = {2}"),
            String::from("journal = {3}"),
            String::from("year = {4}"),
            String::from("volume = {5}"),
            String::from("number = {6}"),
            String::from("pages = {7-10}"),
        ];
        assert_eq!(result, expected);
    }
}
