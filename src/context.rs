use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use tera::Context;
use crate::{CONFIG_FILE, utils::get_stripped_filename};

#[derive(Deserialize, Serialize)]
struct Project {
    name: String,
    description: String,
    link: Option<String>,
    github: String,
}

#[derive(Deserialize, Serialize)]
struct SideProject {
    #[serde(flatten)]
    project: Project
}

#[derive(Deserialize, Serialize)]
struct Config {
    name: String,
    bio: String,
    email: String,
    description: String,
    baseurl: Option<String>,
    projects: Option<Vec<Project>>,
    sideprojects: Option<Vec<SideProject>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>, // un-typed config vars will be stored here
}

pub fn get_config(blog_paths: &Vec<PathBuf>) -> Context
{
    let config = fs::read_to_string(CONFIG_FILE)
        .expect("Error reading configuration (config.yaml) from root dir");

    let yaml : Config = serde_yaml::from_str(config.as_str())
        .expect("Error parsing YAML");

    // un-documented extra vars from config (in case I don't want to modify types 
    // just for adding a new field)
    let extra = yaml.extra.clone();

    let mut context = Context::from_serialize(yaml)
        .expect("Error creating global config context");

    // insert flattened extra vars into context
    for (key, value) in &extra {
        context.insert(key, value);
    }

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