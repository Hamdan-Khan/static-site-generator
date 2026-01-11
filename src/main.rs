mod parser;
mod renderer;
mod utils;
mod context;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use crate::context::get_config;
use crate::parser::parse_content;
use crate::utils::write_file;

// dirs
const CONTENT_DIR : &str = "content";
const STATIC_DIR : &str = "static";
const BUILD_DIR : &str = "public";
// files
const CONFIG_FILE : &str = "config.yaml";

fn main() -> io::Result<()> {
    let base_path = Path::new(CONTENT_DIR);
    let mut file_paths: Vec<PathBuf> = vec![];

    // read content directory and collect md file paths
    for entry in fs::read_dir(base_path)? {
        let path = entry?.path();
        println!("{}",path.display());
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            file_paths.push(path);
        }
    }

    // create a fresh build dir
    fs::remove_dir_all(BUILD_DIR)?;
    fs::create_dir_all(BUILD_DIR)?;


    let config_context = get_config().expect("Error parsing config context");

    // process every md file for blogs
    for p in &file_paths {
        let md_content = fs::read_to_string(p)?;
        
        // parses md content into html
        let (parsed_content, front_matter) = parse_content(md_content.as_str())
            .expect("Error parsing content or front matter");
        
        // construct file name
        let relative_path = p.strip_prefix(&base_path)
           .expect("path not present in the specified content dir");
        let file_name = relative_path.with_extension("html").display().to_string();

        // render parsed content
        let final_html = renderer::render_html(&parsed_content, front_matter, &config_context).
            expect("Couldn't render the content");
        
        write_file( file_name, final_html)?;
    }

    // main file for home page
    let index_page = renderer::render_index(&config_context)
        .expect("Couldn't render home page");
    write_file( "index.html".to_string(), index_page)?;

    // copy static files into build dir
    for static_file in fs::read_dir(STATIC_DIR)? {
        let static_file_path = static_file?.path();
        let relative_path = static_file_path.strip_prefix(&STATIC_DIR)
            .expect("path not present in the specified content dir");
        fs::copy(&static_file_path,format!("{0}/{1}", BUILD_DIR, relative_path.display().to_string()))?;
    }

    Ok(())
}

// todos:
// make file discovery recursive