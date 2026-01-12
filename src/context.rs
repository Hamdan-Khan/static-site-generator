use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tera::Context;
use crate::{CONFIG_FILE, utils::get_stripped_filename};

#[derive(Deserialize, Serialize)]
struct Config {
    name: String,
    bio: String,
    email: String,
    description: String,
    baseurl: Option<String>,
}

pub fn get_config(blog_paths: &Vec<PathBuf>) -> Context
{
    let config = fs::read_to_string(CONFIG_FILE)
        .expect("Error reading configuration (config.yaml) from root dir");

    let yaml : Config = serde_yaml::from_str(config.as_str())
        .expect("Error parsing YAML");

    let mut context = Context::from_serialize(yaml)
        .expect("Error creating global config context");

    // if blog files exist, add their names lists into context to generate nav and stuff
    if blog_paths.len() > 0{
        let mut blogs: Vec<String> = vec![];
        for path in blog_paths {
            blogs.push(get_stripped_filename(path).with_extension("").display().to_string());
        }
        context.insert("blogs", &blogs);
    }

    return context;
}