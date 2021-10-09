use std::{
    fs::{create_dir, remove_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

pub fn write_file(content: &str, path: &PathBuf) -> Result<(), std::io::Error> {
    let mut input_file = File::create(path)?;
    input_file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn prepare_folder<'a>(base_folder: &'a str) -> Result<&'a Path, std::io::Error> {
    let base_folder = Path::new(base_folder);
    if base_folder.exists() {
        remove_dir_all(&base_folder)?;
    }
    create_dir(&base_folder)?;
    Ok(base_folder)
}
