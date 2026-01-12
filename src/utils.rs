use std::{fs, path::{Path, PathBuf}};

use crate::CONTENT_DIR;

/** writes file to build dir */
pub fn write_file(filename: String, contents: String) -> Result<(), std::io::Error>{
    fs::write(format!("{0}/{1}", crate::BUILD_DIR, filename), contents)?;
    Ok(())
}

/** removes the content/ prefix from file name */
pub fn get_stripped_filename(path: &PathBuf) -> &Path{
    let base_path = Path::new(CONTENT_DIR);
    return path.strip_prefix(&base_path)
           .expect("path not present in the specified content dir")
}