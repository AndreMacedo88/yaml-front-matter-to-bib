use article_bio_like::MetadataBio;
use std::error::Error;
use yaml_front_matter::{Document, YamlFrontMatter};
pub mod article_bio_like;

fn parse_document_bio(f: String) -> Result<Document<MetadataBio>, Box<dyn Error>> {
    YamlFrontMatter::parse::<MetadataBio>(&f)
}

pub fn get_yaml_front_matter(
    f: String,
    style: &str,
) -> Result<Document<MetadataBio>, Box<dyn Error>> {
    match style {
        "article_bio_like" => parse_document_bio(f),
        _ => Err("Type not yet implemented".into()),
    }
}

#[cfg(test)]
mod tests_parse_document_bio {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;

    fn test_file_bio_data() -> String {
        let test_f: String = String::from(
            "
---
title: 1
author: 2
journal: 3
year: 4
volume: 5
number: 6
pages: 7-10
---

# test
this is a test
        ",
        );
        test_f
    }

    #[test]
    fn test_title() -> Result<(), Box<dyn Error>> {
        let test_f: String = test_file_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f)?;
        assert_eq!(result.metadata.title, String::from("1"));
        Ok(())
    }

    #[test]
    fn test_author() -> Result<(), Box<dyn Error>> {
        let test_f: String = test_file_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f)?;
        assert_eq!(result.metadata.author, String::from("2"));
        Ok(())
    }

    #[test]
    fn test_journal() -> Result<(), Box<dyn Error>> {
        let test_f: String = test_file_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f)?;
        assert_eq!(result.metadata.journal, String::from("3"));
        Ok(())
    }

    #[test]
    fn test_year() -> Result<(), Box<dyn Error>> {
        let test_f: String = test_file_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f)?;
        assert_eq!(result.metadata.year, 4);
        Ok(())
    }

    #[test]
    fn test_volume() -> Result<(), Box<dyn Error>> {
        let test_f: String = test_file_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f)?;
        assert_eq!(result.metadata.volume, 5);
        Ok(())
    }

    #[test]
    fn test_number() -> Result<(), Box<dyn Error>> {
        let test_f: String = test_file_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f)?;
        assert_eq!(result.metadata.number, 6);
        Ok(())
    }

    #[test]
    fn test_pages() -> Result<(), Box<dyn Error>> {
        let test_f: String = test_file_bio_data();
        let result: Document<MetadataBio> = parse_document_bio(test_f)?;
        assert_eq!(result.metadata.pages, String::from("7-10"));
        Ok(())
    }
}

#[cfg(test)]
mod tests_get_yaml_front_matter {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;

    fn test_file_bio_data() -> String {
        let test_f: String = String::from(
            "
---
title: 1
author: 2
journal: 3
year: 4
volume: 5
number: 6
pages: 7-10
---

# test
this is a test
        ",
        );
        test_f
    }

    #[test]
    fn test_title() -> Result<(), Box<dyn Error>> {
        let test_f: String = test_file_bio_data();
        let style: &str = "article_bio_like";
        let result: Document<MetadataBio> = get_yaml_front_matter(test_f, style)?;
        assert_eq!(result.metadata.title, String::from("1"));
        Ok(())
    }

    #[test]
    fn test_pages() -> Result<(), Box<dyn Error>> {
        let test_f: String = test_file_bio_data();
        let style: &str = "article_bio_like";
        let result: Document<MetadataBio> = get_yaml_front_matter(test_f, style)?;
        assert_eq!(result.metadata.pages, String::from("7-10"));
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_missing_title() -> Result<(), Box<dyn Error>> {
        let test_f: String = String::from(
            "
---
author: 2
journal: 3
year: 4
volume: 5
number: 6
pages: 7-10
---

# test
this is a test
        ",
        );
        let style: &str = "article_bio_like";
        let result: Document<MetadataBio> = get_yaml_front_matter(test_f, style)?;
        assert_eq!(result.metadata.author, String::from("2")); // remove the ignore after implementing
        Ok(())
    }
}
