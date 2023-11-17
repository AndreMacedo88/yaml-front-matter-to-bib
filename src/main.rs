use clap::Parser;
use serde::Deserialize;
use std::error::Error;
use std::fs::{self, metadata, File, OpenOptions};
use std::io::Write;
use std::path::Path;
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

/// A program to gather and turn YAML-formatted front matter of markdown files into a single, correctly formatted, .bib file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(
    help_template = "Running: {name}\nAuthor: {author-with-newline} {about-section}Version: {version} \n\n {usage-heading} {usage} \n {all-args} {tab}"
)]
struct Args {
    /// Path to directory to search for markdown files
    #[arg(short, long)]
    input_directory: String,

    /// Path to store the output .bib file
    #[arg(short, long, default_value = "bibliography.bib")]
    output_path: String,

    /// Overwrites an existing output file instead of appending to it
    #[arg(short = 'O', long, action)]
    overwrite: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let out_path: &Path = Path::new(&args.output_path);

    // create/open a file in the output directory
    let file_result = if out_path.exists() && !args.overwrite {
        OpenOptions::new().append(true).open(out_path)
    } else {
        File::create(out_path)
    };
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => panic!("Unable to open file: {:?}", e),
    };
    // cycle the files and directories in the provided path
    for entry in WalkDir::new(&args.input_directory)
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        // turn the DirEntry object into a Path object
        let path = entry.path();
        // check if the path is pointing to a directory or to a file that is not a markdown
        if metadata(path).unwrap().is_dir()
            || !path.to_str().unwrap().to_lowercase().ends_with(".md")
        {
            continue;
        }
        // read the file and parse the YAML front matter
        let f: String = fs::read_to_string(path)?;
        let parsed_document: Result<Document<Metadata>, Box<dyn Error>> =
            YamlFrontMatter::parse::<Metadata>(&f);
        let yaml_front_matter: Document<Metadata> = match parsed_document {
            Ok(content) => content,
            Err(_) => continue,
        };
        // unpack the front matter to each variable
        let Metadata {
            title,
            author,
            journal,
            year,
            number,
            volume,
            pages,
        } = yaml_front_matter.metadata;
        // get the first author's last name to use as the Key in the .bib format
        let authors: Vec<&str> = author.split("and").collect();
        let first: &str = authors[0].trim();
        let first: Vec<&str> = first.split(" ").collect();
        let last_name: &str = first[first.len() - 1];
        // build the .bib formatted string to write to file
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

        // append these lines to the file
        file.write(output.as_bytes())?;
    }
    Ok(())
}
