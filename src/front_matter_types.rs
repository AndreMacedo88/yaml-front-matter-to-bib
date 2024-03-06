use front_matter_types::article_bio_like::MetadataBio;
use std::error::Error;
use yaml_front_matter::{Document, YamlFrontMatter};
pub mod article_bio_like;
mod front_matter_types;

pub fn parse_document_bio(f: String) {
    let parsed_document: Result<Document<MetadataBio>, Box<dyn Error>> =
        YamlFrontMatter::parse::<MetadataBio>(&f);
    match parsed_document {
        Ok(content) => content,
        Err(_) => continue, // should continue to the next line... how to mantain this behaviour here?
    };
}

// fn article_bio_like_() -> (fn(), fn()) {
//     pub use article_bio_like::{generate_bib_lines, Metadata};
//     (
//         || Metadata {
//             title: String,
//             author: String,
//             journal: String,
//             year: u16,
//             volume: u32,
//             number: u32,
//             pages: String,
//         },
//         generate_bib_lines,
//     )
// }

// pub fn get_type_objects(arg: String) -> (fn(), fn()) {
//     match arg.as_str() {
//         "article_bio_like" => article_bio_like_(),
//         _ => panic!("Type not yet implemented"),
//     }
// }
