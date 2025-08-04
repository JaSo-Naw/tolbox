use crate::code_feeder::files_reader::{MyFile, read_file};
use anyhow::{Context, Result};

pub fn open_folder(folder_path: &str, depth: usize, ignore: &Vec<&String>) -> Result<Vec<MyFile>> {
    let mut file_vec = Vec::new();

    if ignore.contains(&&folder_path.to_string()) {
        return Ok(file_vec);
    }

    file_vec.push(MyFile::new(
        format!("{}", folder_path),
        String::new(),
        depth,
        true,
    ));

    for entry in std::fs::read_dir(folder_path)? {
        let entry = entry.context("failed to read entry")?;
        let path = entry.path();
        let path_str = path.to_str().context("failed to convert path to string")?;

        if path.is_dir() {
            file_vec.append(
                &mut open_folder(path_str, depth + 1, ignore)
                    .context(format!("failed to open {}", path_str))?,
            );
        } else {
            if ignore.contains(&&path_str.to_string()) {
                continue;
            }
            file_vec.push(
                read_file(path_str, depth + 1).context(format!("failed to read {}", path_str))?,
            );
        }
    }
    Ok(file_vec)
}
