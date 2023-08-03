#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::{fs, path};
use tempfile::{tempdir, TempDir};
use walkdir::WalkDir;
use zip::result::ZipResult;
use zip::{ZipArchive, ZipWriter};

mod utils;

type ZipFileReader = zip::ZipArchive<File>;
type ZipFileWriter = zip::ZipWriter<File>;

fn get_file_by_path(path: &str) -> std::io::Result<File> {
    File::options().read(true).write(true).open(path)
}

fn get_zip_reader<R: Read + Seek>(reader: R) -> ZipResult<ZipArchive<R>> {
    ZipArchive::new(reader)
}

// fn get_zip_writer<W: Read + Write + Seek>(writer: W) -> ZipResult<ZipWriter<W>> {
//     ZipWriter::new_append(writer)
// }

#[napi(custom_finalize)]
pub struct Epub {
    reader: ZipFileReader,
    // _writer: ZipFileWriter,
    // path: String,
    temp_dir: TempDir,
}

#[napi]
impl Epub {
    // fn get_writer(&self) -> ZipFileWriter {
    //     let file = get_file_by_path(&self.path.to_owned()).unwrap();

    //     get_zip_writer(file.try_clone().unwrap()).unwrap()
    // }

    #[napi(constructor)]
    pub fn new(path: String) -> Self {
        let file = get_file_by_path(&path.to_owned()).unwrap();

        let temp_dir = tempdir().unwrap();

        let reader = get_zip_reader(file.try_clone().unwrap());
        // let writer = get_zip_writer(file.try_clone().unwrap());

        Epub {
            // path,
            reader: reader.unwrap(),
            // _writer: writer.unwrap(),
            temp_dir,
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
        let mut path = self.temp_dir.path().to_path_buf();
        path.push(file_name);

        let mut file = File::options()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[napi]
    pub fn export_file(&mut self, raw_path: String) -> () {
        let file_path = Path::new(raw_path.as_str());

        let temp_dir = self.temp_dir.path().to_str().unwrap();

        let file = File::create(&file_path)
            .expect("Must specify the name of the epub file, not a directory.");
        let walkdir = WalkDir::new(&temp_dir);
        let mut entries: Vec<_> = walkdir.into_iter().filter_map(|e| e.ok()).collect();

        utils::sort_by_epub_spec(&mut entries);

        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::DEFLATE)
            .unix_permissions(0o755);

        let mut buffer = Vec::new();

        for entry in entries {
            let path = entry.path();
            let name = path.strip_prefix(Path::new(temp_dir)).unwrap();

            if path.is_file() {
                zip.start_file(name.to_str().unwrap().to_owned(), options.clone())
                    .unwrap();
                let mut f = File::open(path).unwrap();

                f.read_to_end(&mut buffer).unwrap();
                zip.write_all(&buffer).unwrap();
                buffer.clear();
            } else if !name.as_os_str().is_empty() {
                zip.start_file(name.to_str().unwrap().to_owned(), options.clone())
                    .unwrap();
            }
        }
        zip.finish().unwrap();
    }

    #[napi]
    pub fn extract(&mut self) -> () {
        let zip_file = &mut self.reader;
        let temp_dir = self.temp_dir.path().to_path_buf();

        for i in 0..zip_file.len() {
            let mut outpath = temp_dir.clone();

            let mut file = zip_file.by_index(i).unwrap();
            let path: PathBuf = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            outpath.push(path.clone());

            if (*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }
}

impl ObjectFinalize for Epub {
    fn finalize(self, mut _env: Env) -> Result<()> {
        self.temp_dir.close()?;
        Ok(())
    }
}
