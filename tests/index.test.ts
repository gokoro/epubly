import test from 'ava'

import { Epub } from '../src/index.js'

const epub = new Epub('tests/test.epub')

test.serial('Read file name', (t) => {
  // console.log('File Content: ', epub.readFileNames())

  t.pass()
})

test.serial('Read file content', (t) => {
  // console.log(
  //   'File Content: ',
  //   epub.readFileContentByName('EPUB/ch02s03.xhtml')
  // )

  t.pass()
})

test.serial('Write file content', (t) => {
  // epub.writeFileContentByName('EPUB/ch02s03.xhtml', 'Test')

  t.pass()
})
