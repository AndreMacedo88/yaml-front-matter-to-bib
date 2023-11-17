use clap::Parser;

/// A program to gather and turn YAML-formatted front matter of markdown files into a single, correctly formatted, .bib file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(
    help_template = "Running: {name}\nAuthor: {author-with-newline} {about-section}Version: {version} \n\n {usage-heading} {usage} \n {all-args} {tab}"
)]
pub struct Args {
    /// Path to directory to search for markdown files
    #[arg(short, long)]
    pub input_directory: String,

    /// Path to store the output .bib file
    #[arg(short, long, default_value = "bibliography.bib")]
    pub output_path: String,

    /// Overwrites an existing output file instead of appending to it
    #[arg(short = 'O', long, action)]
    pub overwrite: bool,
}
