use crate::code_feeder::files_reader::MyFile;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;

pub fn write_file(path: &str, files: Vec<MyFile>) -> Result<()> {
    let mut output = File::create(path).context(format!("failed to create file {path}"))?;

    let mut extra_name = None;

    files.iter().for_each(|f| {
        if extra_name.is_none() {
            extra_name = Some(
                Path::new(&f.name)
                    .parent()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|| String::from("")),
            );
        }

        let indent = "    ".repeat(f.depth);
        if let Some(name) = &extra_name {
            let total_prefix = String::from(name).add("\\");
            let file_name = f
                .name
                .strip_prefix(&total_prefix)
                .expect("failed to strip prefix");
            write!(output, "{}{}\n", indent, file_name).expect("failed to write file");
        }
    });

    files.iter().for_each(|f| {
        if !f.is_dir {
            write!(
                output,
                "------------------------------------------------------------------------\n"
            )
            .unwrap();
            if let Some(name) = &extra_name {
                let total_prefix = String::from(name).add("\\");
                let file_name = f
                    .name
                    .strip_prefix(&total_prefix)
                    .expect("failed to strip prefix");
                write!(output, "{}\n\n", file_name).expect("failed to write file");
                write!(output, "{}\n\n", f.content).expect("failed to write file");
            }
        }
    });

    Ok(())
}
