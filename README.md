# notegen

## Description
When programming, often you encounter a problem which requires you to learn something new in order to solve it. This is great, and provides a way to expand one's knowledge base, but far too often the steps taken to solve the problem get forgotten. When the problem is encountered again, one will inevitably have go back and search through old source code trying to remember how the problem was solved before, or worse, try and remember the convoluted sequence of stack overflow links visited in order to find the solution. This isn't a great use of time.

One way to remedy this would be to take notes on any new technical knowledge gained while programming, where you leave yourself a short description and a code block to jog your memory, but this too is suboptimal. Stopping the flow of problem solving to open a notes application, copy the code over, and write a description that will likely be nearly identical to your source code comments every time something new is discovered is very tedious and hurts productivity.

So, one real solution is to automate it. Write markdown directly in your source code file whenever something worth remembering is done, and let a program handle copying it into a new file to be later viewed by the markdown application of your choice. This way all notes of this type can be in once place, and searched for, not spread over several different projects, buried in their source code.

Currently this program works on Rust, C, Java, and Go files.

## Usage
```
USAGE:
    notegen [OPTIONS] <FILE>

ARGS:
    <FILE>    Name of source code file

OPTIONS:
    -g, --generate-tags    Puts a "#LanguageName" tag after the first regular markdown line (which
                           will usually be a header) for automatic organization in Obsidian
    -h, --help             Print help information
    -t, --tidy-mode        Creates a copy of the source code with a .tidy extension which contains
                           none of the notegen symbols
    -V, --version          Print version information
```


## Building
A binary can be found for Linux and Windows <a href="https://github.com/roomtin/notegen/releases/tag/v0.9.4-alpha">here</a>.

If building from source is preferable, run these commands:
* `git clone https://github.com/roomtin/notegen.git`
* `cd notegen`
* `cargo build --release`

The binary will be in a new directory under `target/release`

Requires that the rust compiler and cargo are installed.

## Symbols
* `//@@` -> Title
* `//@ ` -> Markdown line
* `//@{`, `//@}` -> Code block delimiter

Each note must have a title which is specified with `//@@ <Title>`. This becomes the filename for the new markdown file. Within the source code, use `//@ ` followed by any markdown, and `//@{`,`//@}` to mark a chunk of source code to export as an example into the new markdown file.

It is possible to define multiple markdown files in a single source code file. Each `//@@` tag will create a separate markdown file.

## Source Code Example
<img src="https://user-images.githubusercontent.com/61144046/160654104-19f9e728-124b-450e-96ca-91abf936cdfc.png" alt="Source Image" width="550"/>

## Generated Markdown
<img src="https://user-images.githubusercontent.com/61144046/160656338-f274792d-db0a-4a60-9286-cce827305160.png" alt="Obsidian File" width="550"/>

### Things that I probably ought to add
* python support
* Polish reading in a .config file. Currently is temperamental and not documented.
* ~~Polish tidy mode.~~ 
