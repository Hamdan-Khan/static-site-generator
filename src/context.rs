use std::fs;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use tera::Context;
use crate::parse_content;
use crate::utils::get_stripped_filename;
use crate::{CONFIG_FILE};

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
struct Experience {
    name: String,
    role: String,
    startdate: String,
    enddate: Option<String>,
    location: String,
    description: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
struct ExternalBlog {
    title: String,
    path: String,
    platform: String,
    date: String,
    featured: Option<bool>,
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
    experience: Option<Vec<Experience>>,
    externalblogs: Option<Vec<ExternalBlog>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>, // un-typed config vars will be stored here
}

#[derive(Deserialize, Serialize)]
struct Blog {
    path: String,
    title: String,
    date: Option<String>,
    featured: Option<bool>,
    platform: Option<String>
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
    let external_blogs_opt = yaml.externalblogs.clone();

    let mut context = Context::from_serialize(yaml)
        .expect("Error creating global config context");

    // insert flattened extra vars into context
    for (key, value) in &extra {
        context.insert(key, value);
    }

    // if blog files exist, add their names lists into context to generate nav and stuff
    if blog_paths.len() > 0{
        let mut blogs: Vec<Blog> = vec![];
        for path in blog_paths {
            let md_content = fs::read_to_string(path)
                .expect("Error reading blog file");

            // gets parsed frontmatter from blog contents
            let (_, front_matter) = parse_content(md_content.as_str())
                .expect("Error parsing content or front matter");

            // adds frontmatter metadata to context
            blogs.push(Blog {
                path: get_stripped_filename(path).with_extension("").to_str().unwrap().to_string(),
                title: front_matter.get("title").unwrap().as_str().to_string(),
                date: front_matter.get("date").map(|d| d.as_str().to_string()),
                featured: front_matter.get("featured").map(|d| d.to_lowercase() == "true"),
                platform: None,
            });
        }

        if let Some(external_blogs) = external_blogs_opt {
            for blog in external_blogs {
                blogs.push(Blog {
                    path: blog.path,
                    title: blog.title,
                    date: Some(blog.date),
                    featured: blog.featured,
                    platform: Some(blog.platform),
                });
            }
        }

        // desc order of date published
        blogs.sort_by(|a, b| {
            let date_a = a.date.as_ref().and_then(|d| NaiveDate::parse_from_str(d, "%d-%m-%Y").ok());
            let date_b = b.date.as_ref().and_then(|d| NaiveDate::parse_from_str(d, "%d-%m-%Y").ok());
            date_b.cmp(&date_a)
        });
        context.insert("blogs", &blogs);

        // blogs with featured flag in front matter
        let featured_blogs: Vec<&Blog> = blogs.iter()
            .filter(|blog| blog.featured.unwrap_or(false))
            .collect();
        
        if !featured_blogs.is_empty() {
            context.insert("featured_blogs", &featured_blogs);
        }
    }

    return context;
}