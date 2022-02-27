use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;
use chrono::offset::Utc;
use chrono::DateTime;
use colored::*;
use glob::glob;

struct FileInfo {
    modified: DateTime<Utc>,
    size: String,
    name: PathBuf,
}

fn main() {
    println!("\n{:24}  {:10}  { }", "Date Modified", "Filesize", "Filename");
    println!("------------------------------------------------------------");
    let mut dirs: Vec<FileInfo> = Vec::new();
    let mut files: Vec<FileInfo> = Vec::new();

    for entry in glob("./*").expect("Failed to read directory.") {
        match entry {
            Ok(path) => {
                match path.metadata() {
                    Ok(meta) => {
                        let modtime = meta.modified().expect("Could not get mod time!");
                        let filesize = meta.file_size();
                        let str_filesize = bytes_to_units(filesize);

                        if path.is_dir() {
                            dirs.push(FileInfo { size: "<dir>".to_string(), name: path, modified: modtime.into() });
                        } else {
                            files.push(FileInfo { size: str_filesize, name: path, modified: modtime.into() });
                        }
                    }
                    Err(e) => {
                        println!("{}. File: {}", e, path.to_string_lossy());
                    }
                }
            }
            Err(e) => println!("{}", e),
        }
    }
    for dir in dirs{
        println!("{:24}  {:10}  { }", dir.modified.format("%m/%d/%Y %T"), dir.size, dir.name.to_string_lossy().blue());
    }

    for file in files{
        println!("{:24}  {:10}  { }", file.modified.format("%m/%d/%Y %T"), file.size, file.name.to_string_lossy());
    }
}

fn bytes_to_units(bytes: u64) -> String {
    return match bytes {
        d if d < 1024 => format!("{}B", bytes),
        d if d > 1024 && d < (1024 * 1024) => return format!("{}K", (bytes / 1024)),
        d if d > (1024 * 1024) => return format!("{}MB", (bytes / 1024 / 1024)),
        d if d > (1024 * 1024 * 1024) => return format!("{}GB", (bytes / 1024 / 1024 / 1024)),
        _ => return "".to_string(),
    };
}
