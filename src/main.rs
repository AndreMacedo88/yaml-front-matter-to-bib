use clap::Parser;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::result::Result;
use walkdir::{DirEntry, WalkDir};
use yaml_front_matter::Document;
mod cli;
mod file_handling;
mod front_matter_styles;
mod process_metadata;
use cli::cli::Args;
use file_handling::create_open_output_file;
use front_matter_styles::article_bio_like::{generate_bib_metadata_lines, MetadataBio};
use front_matter_styles::get_yaml_front_matter;
use process_metadata::{get_first_author_last_name, wrap_metadata_lines};

fn main() -> Result<(), Box<dyn Error>> {
    // parse CLI arguments
    let args: Args = Args::parse();

    // create/open a file in the output directory before appending each file's info
    let mut output_handle: File =
        create_open_output_file(args.output_path.as_str(), args.overwrite);

    // cycle the files and directories in the provided path
    for entry in WalkDir::new(&args.input_directory).follow_links(true) {
        // turn the DirEntry object into a Path object
        let entry: DirEntry = entry?;
        let path: &Path = entry.path();

        // Skip directories and non-Markdown files
        if path.is_dir() || !path.extension().map_or(false, |ext| ext == "md") {
            continue;
        }

        // read the file and parse the YAML front matter
        let f: String = fs::read_to_string(path)?;
        let yaml_front_matter: Document<MetadataBio> =
            match get_yaml_front_matter(f, args.style.as_str()) {
                Ok(content) => content,
                Err(_) => continue,
            };

        // get the first author's last name to use as the Key in the .bib format
        let last_name: &str = get_first_author_last_name(&yaml_front_matter.metadata.author)?;

        // build the .bib formatted string to write to file
        let lines: Vec<String> = generate_bib_metadata_lines(&yaml_front_matter.metadata);
        let output: String =
            wrap_metadata_lines(&yaml_front_matter.metadata.year, &last_name, lines);

        // append these lines to the file
        output_handle.write_all(output.as_bytes())?;
    }
    Ok(())
}
