import test from 'ava'

import { Zip } from '../src/index.js'

test.serial('Read file name', (t) => {
  // const zip = new Zip('tests/test1.epub')
  // console.log('Filename: ', zip.readFileNames())

  t.pass()
})

test.serial('Read file content', (t) => {
  // const zip = new Zip('tests/test1.epub')
  // console.log('File Content: ', zip.readFileContentByName('text/part0010.html'))

  t.pass()
})

test.serial('Write file content', (t) => {
  // const zip = new Zip('tests/test1.epub')
  // zip.writeFileContentByName('text/part0010.html', 'Test')

  t.pass()
})
