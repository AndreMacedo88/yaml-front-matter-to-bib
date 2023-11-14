# yaml-header-to-bib

## What does this crate do?

This crate does the following:
- Identifies the YAML section at the start of all markdown files in a directory
- Creates a new bibtex/biblatex file with the parsed references

## Rationale

The motivation behind this crate is so that the user can build their own personal notes on each article/text they read on a minimally formatted document such as markdown.
Then, they can quickly generate a .bib file from all the articles they have read.
This .bib file can then be used as needed e.g. to insert citations in a LaTeX-based dissertation/document.