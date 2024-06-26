use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::error::Error;
use std::fs;

#[test]
fn test_help_command() {
    let mut cmd: Command = Command::cargo_bin("yaml-front-matter-to-bib").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:").and(predicate::str::contains("Options:")));
}

#[test]
fn test_new_bib() -> Result<(), Box<dyn Error>> {
    let mut cmd: Command = Command::cargo_bin("yaml-front-matter-to-bib").unwrap();
    cmd.arg("--input-directory").arg("tests/data");
    cmd.arg("--output-path").arg("tests/data/temp_bib.bib");
    cmd.assert().success();

    // Read the generated .bib file
    let mut generated_bib_content: String = fs::read_to_string("tests/data/temp_bib.bib")?;
    // we strip spaces because they don't mean much in a .bib file
    generated_bib_content.retain(|c: char| !c.is_whitespace());

    // Read the test .bib file
    let mut test_bib_content: String = fs::read_to_string("tests/data/test_bib.bib")?;
    test_bib_content.retain(|c: char| !c.is_whitespace());

    // Delete the generated .bib file
    fs::remove_file("tests/data/temp_bib.bib").expect("Failed to delete generated .bib file");

    // Compare the contents of both files
    assert_eq!(generated_bib_content, test_bib_content);
    Ok(())
}

#[test]
fn test_overwrite_bib() -> Result<(), Box<dyn Error>> {
    let mut cmd: Command = Command::cargo_bin("yaml-front-matter-to-bib").unwrap();
    cmd.arg("--input-directory").arg("tests/data");
    cmd.arg("--output-path").arg("tests/data/temp_bib2.bib");
    cmd.arg("-O");
    cmd.assert().success();
    cmd.assert().success(); // ran twice so that if overwrite is not working, the output will not be correct

    // Read the generated .bib file
    let mut generated_bib_content: String = fs::read_to_string("tests/data/temp_bib2.bib")?;
    // we strip spaces because they don't mean much in a .bib file
    generated_bib_content.retain(|c: char| !c.is_whitespace());

    // Read the test .bib file
    let mut test_bib_content: String = fs::read_to_string("tests/data/test_bib.bib")?;
    test_bib_content.retain(|c: char| !c.is_whitespace());

    // Delete the generated .bib file
    fs::remove_file("tests/data/temp_bib2.bib").expect("Failed to delete generated .bib file");

    // Compare the contents of both files
    assert_eq!(generated_bib_content, test_bib_content);
    Ok(())
}
