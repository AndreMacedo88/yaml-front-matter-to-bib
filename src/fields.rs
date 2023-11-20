use serde::Deserialize;
use yaml_front_matter::Document;

#[derive(Deserialize)]
pub struct Metadata {
    pub title: String,
    pub author: String,
    pub journal: String,
    pub year: u16,
    pub volume: u32,
    pub number: u32,
    pub pages: String,
}

pub fn create_bio_like_articles<'a>(metadata: &Document<Metadata>) -> Vec<String> {
    let metadata: &Metadata = &metadata.metadata;
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
