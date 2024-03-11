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

    #[test]
    fn test_title() {
        let test_f: String = test_parse_document_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f).unwrap();
        assert_eq!(result.metadata.title, String::from("1"));
    }

    #[test]
    fn test_author() {
        let test_f: String = test_parse_document_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f).unwrap();
        assert_eq!(result.metadata.author, String::from("2"));
    }

    #[test]
    fn test_journal() {
        let test_f: String = test_parse_document_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f).unwrap();
        assert_eq!(result.metadata.journal, String::from("3"));
    }

    #[test]
    fn test_year() {
        let test_f: String = test_parse_document_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f).unwrap();
        assert_eq!(result.metadata.year, 4);
    }

    #[test]
    fn test_volume() {
        let test_f: String = test_parse_document_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f).unwrap();
        assert_eq!(result.metadata.volume, 5);
    }

    #[test]
    fn test_number() {
        let test_f: String = test_parse_document_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f).unwrap();
        assert_eq!(result.metadata.number, 6);
    }

    #[test]
    fn test_pages() {
        let test_f: String = test_parse_document_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f).unwrap();
        assert_eq!(result.metadata.pages, String::from("7"));
    }
}
