use serde::Deserialize;
use std::fs;
use std::fs::metadata;
// use walkdir::{DirEntry, WalkDir};
use walkdir::WalkDir;
use yaml_front_matter::{Document, YamlFrontMatter};

#[derive(Deserialize)]
struct Metadata {
    title: String,
    author: String,
    journal: String,
    year: u16,
    number: u32,
    volume: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path: &str = "/home/andrem/Documents/notes_science/";
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

        let document_result: Result<Document<Metadata>, Box<dyn std::error::Error>> =
            YamlFrontMatter::parse::<Metadata>(&f);

        let document: Document<Metadata> = match document_result {
            Ok(yaml_front_matter) => yaml_front_matter,
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
        } = document.metadata;

        let authors: Vec<&str> = author.split("and").collect();
        let first: &str = authors[0].trim();
        let first: Vec<&str> = first.split(" ").collect();
        let last_name: &str = first[first.len() - 1];

        let _output: String = format!(
            "@article{{{last_name}{year},
        title = {{{title}}},
        author = {{{author}}},
        journal = {{{journal}}},
        year = {{{year}}},
        number = {{{number}}},
        volume = {{{volume}}}
        }}"
        );

        let _out_path: &str = "/home/andrem/Documents/notes_science/test.bib";

        // fs::write(out_path, output).expect("Unable to write file.");
    }
    Ok(())
}
