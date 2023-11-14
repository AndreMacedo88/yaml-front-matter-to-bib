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

    let output: String = format!(
        "@article{{{author}{year},
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
