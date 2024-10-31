use std::fs;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use rayon::prelude::*;
use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let src_dir = "C:\\Users\\marcio\\deixei\\Pictures";
    let dest_dir = "C:\\dev\\temp";

    // Step 1: Gather metadata and sort files
    let mut file_map: HashMap<String, Vec<String>> = HashMap::new();

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Ok(metadata) = fs::metadata(&path) {
                let created = metadata.created().unwrap_or(SystemTime::now());
                let datetime: DateTime<Local> = created.into();
                let date_str = datetime.format("%Y/%m/%Y%m%d").to_string();

                file_map.entry(date_str).or_insert_with(Vec::new).push(path.to_string_lossy().to_string());
            }
        }
    }

    // Step 2: Perform multi-threaded file copy
    file_map.par_iter().for_each(|(date_str, files)| {
        let dest_path = Path::new(dest_dir).join(date_str);
        if let Err(e) = fs::create_dir_all(&dest_path) {
            eprintln!("Failed to create directory: {}", e);
            return;
        }

        files.par_iter().for_each(|file_path| {
            let path = Path::new(file_path);
            let file_name = path.file_name().unwrap();
            let dest_file_path = dest_path.join(file_name);

            if let Err(e) = fs::copy(&path, &dest_file_path) {
                eprintln!("Failed to copy file: {}", e);
            }
        });
    });

    Ok(())
}
