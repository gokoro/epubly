#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs::File;
use std::io::{Read, Seek, Write};
use zip::result::ZipResult;
use zip::{ZipArchive, ZipWriter};

type ZipFileReader = zip::ZipArchive<File>;
type ZipFileWriter = zip::ZipWriter<File>;

fn get_file_by_path(path: &str) -> std::io::Result<File> {
    File::options().read(true).write(true).open(path)
}

fn get_zip_reader<R: Read + Seek>(reader: R) -> ZipResult<ZipArchive<R>> {
    ZipArchive::new(reader)
}

fn get_zip_writer<W: Read + Write + Seek>(writer: W) -> ZipResult<ZipWriter<W>> {
    ZipWriter::new_append(writer)
}

#[napi]
pub struct Epub {
    reader: ZipFileReader,
    _writer: ZipFileWriter,
    path: String,
}

#[napi]
impl Epub {
    fn get_writer(&self) -> ZipFileWriter {
        let file = get_file_by_path(&self.path.to_owned()).unwrap();

        get_zip_writer(file.try_clone().unwrap()).unwrap()
    }

    #[napi(constructor)]
    pub fn new(path: String) -> Self {
        let file = get_file_by_path(&path.to_owned()).unwrap();

        let reader = get_zip_reader(file.try_clone().unwrap());
        let writer = get_zip_writer(file.try_clone().unwrap());

        Epub {
            path,
            reader: reader.unwrap(),
            _writer: writer.unwrap(),
        }
    }

    #[napi]
    pub fn read_file_names(&mut self) -> Vec<String> {
        let zip = &mut self.reader;

        zip.file_names().map(String::from).collect()
    }

    #[napi]
    pub fn read_file_content_by_name(&mut self, file_name: String) -> String {
        let file = &mut self.reader.by_name(&file_name).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        contents
    }

    #[napi]
    pub fn write_file_content_by_name(&mut self, file_name: String, content: String) -> () {
        let mut writer = self.get_writer();

        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::DEFLATE)
            .unix_permissions(0o755)
            .large_file(true);

        writer.start_file(file_name, options).unwrap();
        writer.write_all(content.as_bytes()).unwrap();

        writer.finish().unwrap();
    }

    // #[napi]
    // pub fn _extract(&mut self, _path: String) -> () {
    //     let zip_file = &mut self.reader;

    //     for i in 0..zip_file.len() {
    //         let mut file = zip_file.by_index(i).unwrap();
    //         let outpath = match file.enclosed_name() {
    //             Some(path) => path.to_owned(),
    //             None => continue,
    //         };

    //         if (*file.name()).ends_with('/') {
    //             println!("File {} extracted to \"{}\"", i, outpath.display());
    //             fs::create_dir_all(&outpath).unwrap();
    //         } else {
    //             println!(
    //                 "File {} extracted to \"{}\" ({} bytes)",
    //                 i,
    //                 outpath.display(),
    //                 file.size()
    //             );
    //             if let Some(p) = outpath.parent() {
    //                 if !p.exists() {
    //                     fs::create_dir_all(p).unwrap();
    //                 }
    //             }
    //             let mut outfile = fs::File::create(&outpath).unwrap();
    //             std::io::copy(&mut file, &mut outfile).unwrap();
    //         }
    //     }
    // }
}
