# scorn

Basic blog engine built for minimal size, ease of use and flexibility.

## Installation

`git clone https://github.com/jbruws/scorn.git && cd scorn && cargo run`

The website will be served at `localhost:7117`.

## Usage

Place Markdown files **(with .md extension!)** in the `./markdown` folder, and they will be automatically converted to HTML, indexed and served on the website.

Basic Markdown formatting (bold/italic text, codeblocks, headings, horizontal rules, etc.) is supported. To embed images, use `[!path]`, where `path` is either a path to a locally stored image in `./markdown` or a direct link to an image on another site.

## License

Scorn is licenced under Fight Club License.
