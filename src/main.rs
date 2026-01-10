use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("content");
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        println!("Name: {}", path.display());
    }

    Ok(())
}