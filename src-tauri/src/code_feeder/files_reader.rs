use anyhow::{Context, Result};
use std::io::{BufReader, Read};
use std::path::Path;

pub struct MyFile {
    pub name: String,
    pub content: String,

    pub depth: usize,
    pub is_dir: bool,
}

impl MyFile {
    pub fn new(name: String, content: String, depth: usize, is_dir: bool) -> MyFile {
        MyFile {
            name,
            content,
            depth,
            is_dir,
        }
    }
}

pub fn read_file(path: &str, depth: usize) -> Result<MyFile> {
    let input = std::fs::File::open(path).context(format!("failed to open file {path}"))?;
    let mut buffered = BufReader::new(input);

    let content = if is_binary_file(path) {
        String::new()
    } else {
        let mut c = String::new();
        buffered
            .read_to_string(&mut c)
            .context(format!("failed to read file {path}"))?;
        c
    };

    Ok(MyFile::new(path.to_string(), content, depth, false))
}

fn is_binary_file(path: &str) -> bool {
    let path = Path::new(path);
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(
            ext.as_str(),
            "png"
                | "jpg"
                | "jpeg"
                | "gif"
                | "bmp"
                | "ico"
                | "tiff"
                | "webp"
                | "psd"
                | "raw"
                | "heif"
                | "svg"
                | "exe"
                | "dll"
                | "so"
                | "bin"
                | "zip"
                | "rar"
                | "tar"
                | "gz"
                | "7z"
                | "pdf"
                | "doc"
                | "docx"
                | "xls"
                | "xlsx"
                | "ppt"
                | "pptx"
                | "odt"
        )
    } else {
        false
    }
}
