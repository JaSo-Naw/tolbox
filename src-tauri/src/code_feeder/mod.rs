use anyhow::Result;
use serde::{Deserialize, Serialize};

mod file_writer;
mod files_reader;
mod folder_opener;

use crate::code_feeder::file_writer::write_file;
use crate::code_feeder::folder_opener::open_folder;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    success: bool,
    message: String,
}

pub fn process_folder(folder: &str, output: &str, ignore: Vec<String>) -> Result<ProcessResult> {
    let ignore_refs: Vec<&String> = ignore.iter().collect();

    match open_folder(folder, 0, &ignore_refs) {
        Ok(files) => match write_file(output, files) {
            Ok(_) => Ok(ProcessResult {
                success: true,
                message: "Folder processed successfully".to_string(),
            }),
            Err(e) => Ok(ProcessResult {
                success: false,
                message: format!("Failed to write output: {}", e),
            }),
        },
        Err(e) => Ok(ProcessResult {
            success: false,
            message: format!("Failed to process folder: {}", e),
        }),
    }
}

#[tauri::command]
pub fn process_folder_command(
    folder: String,
    output: String,
    ignore: Vec<String>,
) -> ProcessResult {
    process_folder(&folder, &output, ignore).unwrap_or_else(|e| ProcessResult {
        success: false,
        message: format!("Unexpected error: {}", e),
    })
}
