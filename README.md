# rile-cli

A CLI tool written in Rust that allows users to query files using natural language.

## Installation

### From source

If you have Rust installed, you can install Rile directly from the source:

`cargo install --git https://github.com/yourusername/rile.git`

### Binary releases

Check the [releases page](https://github.com/AbhiByte/rile-cli/releases) for pre-compiled binaries for your platform.

## Usage (Very limited as I build out several features)

`rile --file <FILE_EXTENSION>`

For example, to search for all PDF files:
`rile --file pdf`

You can also specifiy multiple file extensions:
`rile --file .pdf .py .yml`

Tip: Both `.docx` and `docx` work!

## License

This project is licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
