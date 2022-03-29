# notegen

#### Automatically generate markdown files from a source code files containing embedded markdown notes

## Usage
```
USAGE:
    notegen [OPTIONS] <FILE>

ARGS:
    <FILE>    Name of source code file

OPTIONS:
    -g, --generate-tags    Puts a "#LanguageName" tag at the end of the file for automatic
                           organization in Obsidian
    -h, --help             Print help information
    -V, --version          Print version information
```

## Symbols
* `//@@` -> Title
* `//@ ` -> Markdown line
* `//@{`, `//@}` -> Code block delimiter

Each note must have a title which is specified with `//@@ <Title>`. This becomes the filename for the new markdown file. Within the source code, use `//@ ` followed by any markdown, and `//@{`,`//@}` to mark a chunk of source code to export as an example into the new markdown file.

## Source Code Example
<img src="https://user-images.githubusercontent.com/61144046/160654104-19f9e728-124b-450e-96ca-91abf936cdfc.png" alt="Source Image" width="550"/>

## Generated Markdown
<img src="https://user-images.githubusercontent.com/61144046/160656338-f274792d-db0a-4a60-9286-cce827305160.png" alt="Obsidian File" width="550"/>
