import test from 'ava'

import { Epub } from '../src/index.js'

const epub = new Epub('./tests/test.epub')

test.serial('Read file name', (t) => {
  epub.readFileNames()
  t.pass()
})

test.serial('Read file content', (t) => {
  epub.readFileContentByName('EPUB/xhtml/introduction.xhtml')
  t.pass()
})

test.serial('Write file content', (t) => {
  epub.extract()
  epub.exportFile('./tests/test-modified.epub')

  t.pass()
})
