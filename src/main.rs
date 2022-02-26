use std::os::windows::fs::MetadataExt;
use chrono::offset::Utc;
use chrono::DateTime;
use colored::*;

use glob::glob;

fn main() {
    // let args: Vec<String> = env::args().collect();
    println!("\n{:20}  {:12}  { }", "Date Modified", "Filesize", "Filename");
    println!("------------------------------------------------------------");

    for entry in glob("./*").expect("Failed to read directory.") {
        match entry{
            Ok(path) => {
                let filesize = path.metadata().unwrap().file_size();
                let system_time = path.metadata().unwrap().modified().unwrap();
                let datetime: DateTime<Utc> = system_time.into();
                let str_filesize = bytes_to_units(filesize);
                if path.is_dir() {
                    println!("{:12}  {:12}  { }", datetime.format("%m/%d/%Y %T"), "<dir>", path.file_name().unwrap().to_string_lossy().blue());
                } else {
                    println!("{:12}  {:12}  { }", datetime.format("%m/%d/%Y %T"), str_filesize, path.file_name().unwrap().to_string_lossy());
                }

            },
            Err(e) => println!("{:?}", e),
        }
    }

}

fn bytes_to_units(bytes: u64) -> Box<str> {
    return match bytes {
        d if d < 1024 => Box::from(format!("{}B", bytes)),
        d if d > 1024 && d < (1024 * 1024) => return Box::from(format!("{}K", (bytes/1024))),
        d if d > (1024 * 1024) => return Box::from(format!("{}MB", (bytes/1024/1024))),
        d if d > (1024 * 1024 * 1024) => return Box::from(format!("{}GB", (bytes/1024/1024/1024))),
        _ => return Box::from(""),
    }
}
