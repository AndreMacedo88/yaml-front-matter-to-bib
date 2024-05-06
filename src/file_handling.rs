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
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_output_created() -> Result<(), Box<dyn Error>> {
        let output_path_str: &str = "test1.bib";
        let file: File = create_open_output_file(output_path_str, true);

        // read file to an object
        let condition: bool = file.metadata().is_ok();

        // remove test file
        fs::remove_file(output_path_str)?;

        // test if file was created
        assert!(condition);
        Ok(())
    }

    #[test]
    fn test_output_appended() -> Result<(), Box<dyn Error>> {
        let output_path_str: &str = "test2.bib";

        // create a file to append
        let mut file: File = File::create(output_path_str)?;
        file.write("Test. ".as_bytes())?;
        drop(file);

        // append more bytes to that file
        let mut file2: File = create_open_output_file(output_path_str, false);
        file2.write("Test2.".as_bytes())?;

        // read file to an object
        let contents: String = fs::read_to_string(output_path_str)?;

        // remove test file
        fs::remove_file(output_path_str)?;

        // test if the text was appended
        assert_eq!(contents, String::from("Test. Test2."));
        Ok(())
    }

    #[test]
    fn test_output_overwritten() -> Result<(), Box<dyn Error>> {
        let output_path_str: &str = "test3.bib";

        // create a file and write some text
        let mut file: File = File::create(output_path_str)?;
        file.write("Test. ".as_bytes())?;
        drop(file);

        // overwrite some bytes on that file
        let mut file2: File = create_open_output_file(output_path_str, true);
        file2.write("Test2.".as_bytes())?;

        // read file to an object
        let contents: String = fs::read_to_string(output_path_str)?;

        // remove test file
        fs::remove_file(output_path_str)?;

        // test if the text was overwritten
        assert_eq!(contents, String::from("Test2."));
        Ok(())
    }
}
