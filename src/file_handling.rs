use std::fs::{File, OpenOptions};
use std::path::Path;

pub fn create_open_output_file(output: &str, overwrite: bool) -> File {
    let out_path: &Path = Path::new(output);
    let file_result: Result<File, std::io::Error> = if overwrite {
        File::create(out_path)
    } else {
        OpenOptions::new().append(true).create(true).open(out_path)
    };

    match file_result {
        Ok(f) => f,
        Err(e) => panic!("Unable to open file: {:?}", e),
    }
}

#[cfg(test)]
mod tests_create_open_output_file {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_output_created() {
        let output_path_str: &str = "test1.bib";
        let file: File = create_open_output_file(output_path_str, true);
        // test if file was created
        assert!(file.metadata().is_ok());
        // remove test file
        fs::remove_file(output_path_str).unwrap();
    }

    #[test]
    fn test_output_appended() {
        let output_path_str: &str = "test2.bib";

        // create a file to append
        let mut file: File = File::create(output_path_str).unwrap();
        file.write("Test. ".as_bytes()).unwrap();
        drop(file);

        // append more bytes to that file
        let mut file2: File = create_open_output_file(output_path_str, false);
        file2.write("Test2.".as_bytes()).unwrap();

        // test if the text was appended
        let contents: String = fs::read_to_string(output_path_str).unwrap();
        assert_eq!(contents, String::from("Test. Test2."));

        // remove test file
        fs::remove_file(output_path_str).unwrap();
    }

    #[test]
    fn test_output_overwritten() {
        let output_path_str: &str = "test3.bib";

        // create a file and write some text
        let mut file: File = File::create(output_path_str).unwrap();
        file.write("Test. ".as_bytes()).unwrap();
        drop(file);

        // overwrite some bytes on that file
        let mut file2: File = create_open_output_file(output_path_str, true);
        file2.write("Test2.".as_bytes()).unwrap();

        // test if the text was overwritten
        let contents: String = fs::read_to_string(output_path_str).unwrap();
        assert_eq!(contents, String::from("Test2."));

        // remove test file
        fs::remove_file(output_path_str).unwrap();
    }
}
