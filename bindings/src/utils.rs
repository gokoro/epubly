use walkdir::DirEntry;

pub fn sort_by_epub_spec(entries: &mut Vec<DirEntry>) {
    entries.sort_by(|a, b| {
        let a_is_mimetype = a.file_name().to_string_lossy() == "mimetype";
        let b_is_mimetype = b.file_name().to_string_lossy() == "mimetype";

        match (a_is_mimetype, b_is_mimetype) {
            (true, true) | (false, false) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
        }
    });
}
