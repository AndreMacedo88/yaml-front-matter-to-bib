use article_bio_like::MetadataBio;
use std::error::Error;
use yaml_front_matter::{Document, YamlFrontMatter};
pub mod article_bio_like;

pub fn parse_document_bio(f: String) {
    let parsed_document: Result<Document<MetadataBio>, Box<dyn Error>> =
        YamlFrontMatter::parse::<MetadataBio>(&f);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
