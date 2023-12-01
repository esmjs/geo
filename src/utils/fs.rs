use serde_json;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn read_previous_json_file(
    filename: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if fs::metadata(filename).is_ok() {
        let content: String = fs::read_to_string(filename)?;
        Ok(Some(content))
    } else {
        Ok(None)
    }
}

pub fn write_to_json_file(
    filename: &str,
    content: &str,
    pretty_print: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let file: File = File::create(filename)?;
    let mut writer: std::io::BufWriter<_> = std::io::BufWriter::new(file);

    let json_value: serde_json::Value = serde_json::from_str::<serde_json::Value>(content)?;
    let json_string: String = if pretty_print {
        serde_json::to_string_pretty(&json_value)?
    } else {
        serde_json::to_string(&json_value)?
    };

    writer.write_all(json_string.as_bytes())?;
    Ok(())
}

pub fn is_json_content_same(file_content: &str, new_content: &str) -> bool {
    let file_json: serde_json::Value = serde_json::from_str(file_content).unwrap();
    let new_json: serde_json::Value = serde_json::from_str(new_content).unwrap();

    file_json == new_json
}

pub fn create_folder_if_not_exists<P: AsRef<Path>>(path: P) -> io::Result<()> {
    match fs::create_dir(path.as_ref()) {
        Ok(()) => Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        err @ Err(_) => err,
    }
}
