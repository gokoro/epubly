# Epubly

<a href="https://www.npmjs.com/package/@epubly/core" alt="NPM URL of epubly">
<img alt="NPM badge" src="https://img.shields.io/npm/v/%40epubly/core?style=flat-square" />
</a>

Epubly is a blazingly-fast, Rust-powered Node.js library for EPUB editing. With its powerful and flexible API, Epubly simplifies and streamlines the process of manipulating EPUB files.

## Installation

Install via NPM:

```
npm install @epubly/core
```

## Usage

```javascript
import { Epub } from '@epubly/core'

const epub = new Epub('./my-book.epub')

// Read filenames from epub
epub.readFileNames()

// Read the contens of specific file
epub.readFileContentByName('intro.xhtml')

// Write to epub file and export the epub
// You need to call the `epub.extract()` first.
epub.extract()
epub.epub.writeFileContentByName('intro.xhtml', 'Hello 😀')
epub.exportFile('./modified.epub')
```

> [!NOTE]  
> This package is designed for native ESM and don't support for CommonJS exports.

## Contributing to Epubly

To contribute to Epubly, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b '<branch_name>'`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`.
4. Push to the original branch: `git push origin '<project_name>/<location>'`.
5. Create the pull request.

Alternatively see the GitHub documentation on [creating a pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request).

## License

This library is under MIT.
