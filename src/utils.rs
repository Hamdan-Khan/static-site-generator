use std::fs;

/** writes file to build dir */
pub fn write_file(filename: String, contents: String) -> Result<(), std::io::Error>{
    fs::write(format!("{0}/{1}", crate::BUILD_DIR, filename), contents)?;
    Ok(())
}
