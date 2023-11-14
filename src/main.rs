use serde::Deserialize;
use std::fs;
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
    let file_path: &str = "/home/andrem/Documents/notes_science/APA/The_predicted_RNA-binding_protein_regulome_of_axonal_mRNAs.md";

    let f: String = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let document: Document<Metadata> = YamlFrontMatter::parse::<Metadata>(&f).unwrap();

    let Metadata {
        title,
        author,
        journal,
        year,
        number,
        volume,
    } = document.metadata;

    // let (author1, ss): (String, Vec<_>) = author.split("and").collect();
    let authors: Vec<&str> = author.split("and").collect();
    let first: &str = authors[0].trim();
    let first: Vec<&str> = first.split(" ").collect();
    let last_name: &str = first[first.len() - 1];
    println!("{}", last_name);

    let output: String = format!(
        "@article{{{last_name}{year},
    title={{{title}}},
    author={{{author}}},
    journal={{{journal}}},
    year={{{year}}},
    number={{{number}}},
    volume={{{volume}}}
    }}"
    );

    let out_path: &str = "/home/andrem/Documents/notes_science/test.bib";

    fs::write(out_path, output).expect("Unable to write file");
    println!("");
    Ok(())
}
