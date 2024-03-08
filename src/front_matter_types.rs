use article_bio_like::MetadataBio;
use std::error::Error;
use yaml_front_matter::{Document, YamlFrontMatter};
pub mod article_bio_like;

pub fn parse_document_bio(f: String) -> Result<Document<MetadataBio>, Box<dyn Error>> {
    let parsed_document: Result<Document<MetadataBio>, Box<dyn Error>> =
        YamlFrontMatter::parse::<MetadataBio>(&f);
    parsed_document
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_document_bio_data() -> String {
        let test_f: String = String::from(
            "
---
title: 1
author: 2
journal: 3
year: 4
volume: 5
number: 6
pages: 7
---

# test
this is a test
        ",
        );
        test_f
    }

    fn test_title() {
        let test_f: String = test_parse_document_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f).unwrap();
        assert_eq!(result.metadata.title, String::from("1"));
    }
}

// let expected_result = MetadataBio {
//     title: String::from("1"),
//     author: String::from("2"),
//     journal: String::from("3"),
//     year: u16::from(4),
//     volume: u32::from(5),
//     number: u32::from(6),
//     pages: String::from("7"),
// };
