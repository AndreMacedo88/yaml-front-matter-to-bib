use serde::Deserialize;
use yaml_front_matter::Document;

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

pub fn generate_bib_lines<'a>(metadata: &Document<MetadataBio>) -> Vec<String> {
    let metadata: &MetadataBio = &metadata.metadata;
    vec![
        format!("title = {{{}}}", metadata.title),
        format!("author = {{{}}}", metadata.author),
        format!("journal = {{{}}}", metadata.journal),
        format!("year = {{{}}}", metadata.year),
        format!("number = {{{}}}", metadata.number),
        format!("volume = {{{}}}", metadata.volume),
        format!("pages = {{{}}}", metadata.pages),
    ]
}
