# yaml-header-to-bib
## What does this package do?
This tool does the following:
- Identifies the YAML section at the start of all markdown files in a directory
- Creates a new bibtex/biblatex file with the parsed references

## Rationale
The motivation behind this tool is so that the user can build their own personal notes on each article/text they read on a minimally formatted document such as markdown.
Then, they can quickly generate a .bib file from all the articles they have read.
The resulting .bib file can then be used as needed e.g. to insert citations in a LaTeX-based dissertation/document.

## Format
The tool recognizes the following fields and turns them to .bib format:

```yaml
---
title: example
author: example
journal: example
year: example
volume: example
number: example
pages: example 
---
```

## Fields
- title: The title of the work
- author: the authors, using their full name `First [Middle] Last`, with each author separated by `and`
- journal: The journal where it was published the work
- year: The year of publication
- volume: the volume number of the journal where it was published. Usually the `31` in the AMA formatted citation: `... 2022;31(4):324-328`
- number: the issue of the journal where it was published. Usually the `4` in the AMA formatted citation: `... 2022;31(4):324-328`.
The reason for this is that in the .bib format `issue` has a different [meaning](https://ctan.ebinger.cc/tex-archive/macros/latex/contrib/biblatex/doc/biblatex.pdf)
- pages: the page number in the format `int`, or `int-int`. If the journal reports the article number instead of pages, this number comes here.
For journals with page number, it's usually the `324-328` in the AMA formatted citation: `... 2022;31(4):324-328`
For journals with article number, it's Usually the `123` in the AMA formatted citation: `... 2022;31(4):123`