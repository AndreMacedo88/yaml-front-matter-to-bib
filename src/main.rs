use serde::Deserialize;
use std::error::Error;
use std::fs::metadata;
use std::fs::{self, File};
use std::io::Write;
use std::result::Result;
use walkdir::WalkDir;
use yaml_front_matter::{Document, YamlFrontMatter};

#[derive(Deserialize)]
struct Metadata {
    title: String,
    author: String,
    journal: String,
    year: u16,
    volume: u32,
    number: u32,
    pages: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_path: &str = "/home/andrem/Documents/notes_science/test.bib";
    let dir_path: &str = "/home/andrem/Documents/notes_science/";
    // Create a file
    let file_result = File::create(out_path);

    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => panic!("Unable to open file: {:?}", e),
    };

    for entry in WalkDir::new(dir_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        // turn the DirEntry object into a Path object
        let path = entry.path();

        // check if the path is to a directory or to a file that is not a markdown
        if metadata(path).unwrap().is_dir() {
            continue;
        }
        if !path.to_str().unwrap().ends_with(".md") {
            continue;
        }

        let f: String = fs::read_to_string(path).expect("Should have been able to read the file");

        let parsed_document: Result<Document<Metadata>, Box<dyn Error>> =
            YamlFrontMatter::parse::<Metadata>(&f);

        let yaml_front_matter: Document<Metadata> = match parsed_document {
            Ok(content) => content,
            Err(_) => continue,
        };

        println!("{}", path.display());

        let Metadata {
            title,
            author,
            journal,
            year,
            number,
            volume,
            pages,
        } = yaml_front_matter.metadata;

        let authors: Vec<&str> = author.split("and").collect();
        let first: &str = authors[0].trim();
        let first: Vec<&str> = first.split(" ").collect();
        let last_name: &str = first[first.len() - 1];

        let output: String = format!(
            "@article{{{last_name}{year},
    title = {{{title}}},
    author = {{{author}}},
    journal = {{{journal}}},
    year = {{{year}}},
    number = {{{number}}},
    volume = {{{volume}}},
    pages = {{{pages}}}
}}
"
        );

        file.write(output.as_bytes())?;
    }
    Ok(())
}
