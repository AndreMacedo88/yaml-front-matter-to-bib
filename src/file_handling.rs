use std::fs::{File, OpenOptions};
use std::path::Path;

pub fn create_open_output_file(output: &str, overwrite: bool) -> File {
    let out_path: &Path = Path::new(output);
    let file_result: Result<File, std::io::Error> = if out_path.exists() && !overwrite {
        OpenOptions::new().append(true).open(out_path)
    } else {
        File::create(out_path)
    };
    let file: File = match file_result {
        Ok(f) => f,
        Err(e) => panic!("Unable to open file: {:?}", e),
    };
    file
}
