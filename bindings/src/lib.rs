#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::{Read, Seek, Write};

use zip::ZipWriter;

type ZipFileBufferReader = BufReader<File>;
type ZipFileBufferWriter = BufWriter<File>;
type ZipFileArchive = zip::ZipArchive<ZipFileBufferReader>;
type ZipFileWriter = zip::ZipWriter<File>;

fn get_file_buffer(
    path: &String,
) -> std::io::Result<(ZipFileBufferReader, zip::ZipWriter<std::fs::File>)> {
    let file: File = File::options().read(true).write(true).open(path)?;

    Ok((
        BufReader::new(file.try_clone()?),
        zip::ZipWriter::new_append(file.try_clone()?).unwrap(),
    ))
}

fn get_file_struct_by_buf(path: &String) -> std::io::Result<File> {
    fs::OpenOptions::new().read(true).write(true).open(path)
}

fn get_by_path(path: &String) -> (ZipFileArchive, ZipFileWriter) {
    let (reader_buf, writer_buf) = get_file_buffer(path).unwrap();
    let zip_file_reader: ZipFileArchive = zip::ZipArchive::new(reader_buf).unwrap();
    let zip_file_writer = writer_buf;

    (zip_file_reader, zip_file_writer)
}

fn make_dir_writable(path: &str) -> std::io::Result<()> {
    let mut permissions = fs::metadata(path)?.permissions();
    // let new_permissions = fs::Permissions::from_mode(permissions.mode() | 0o777); // add write permission
    permissions.set_readonly(false);

    fs::set_permissions(path, permissions)
}

fn get_file_by_path(path: &str) -> std::io::Result<File> {
    fs::OpenOptions::new().read(true).write(true).open(path)
}

fn get_zip_reader(file: File) -> zip::result::ZipResult<zip::ZipArchive<File>> {
    zip::ZipArchive::new(file)
}

fn get_zip_writer(file: File) -> zip::result::ZipResult<zip::ZipWriter<File>> {
    zip::ZipWriter::new_append(file)
}

#[napi(js_name = "Zip")]
pub struct JsZip {
    zip_archive: ZipFileArchive,
    zip_writer: ZipFileWriter,
    path: String,
}

#[napi]
impl JsZip {
    #[napi(constructor)]
    pub fn new(path: String) -> Self {
        let (zip_archive, zip_writer) = get_by_path(&path);

        JsZip {
            zip_archive,
            zip_writer,
            path,
        }
    }

    #[napi]
    pub fn read_file_names(&mut self) -> Vec<String> {
        let zip = &mut self.zip_archive;

        zip.file_names().map(String::from).collect()
    }

    #[napi]
    pub fn read_file_content_by_name(&mut self, file_name: String) -> String {
        let file = &mut self.zip_archive.by_name(&file_name).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        contents
    }

    #[napi]
    pub fn write_file_content_by_name(&mut self, file_name: String, content: String) -> () {
        let (_, mut zip) = get_by_path(&self.path);
        // let zip = &mut self.zip_writer;

        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755)
            .large_file(true);

        zip.start_file(file_name, options).unwrap();
        zip.write_all(content.as_bytes()).unwrap();

        zip.finish().unwrap();
    }

    pub fn get_zip_writer(path: &String) -> zip::result::ZipResult<ZipWriter<File>> {
        let file = get_file_struct_by_buf(path).unwrap();
        zip::ZipWriter::new_append(file)
    }

    #[napi]
    pub fn _extract(&mut self, path: String) -> () {
        let zip_file: &mut zip::ZipArchive<BufReader<File>> = &mut self.zip_archive;

        for i in 0..zip_file.len() {
            let mut file = zip_file.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if (*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.display());
                fs::create_dir_all(&outpath).unwrap();
            } else {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }
}
