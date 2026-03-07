use lazy_static::lazy_static;
use std::collections::HashMap;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

pub fn render_blogs_list(config_context: &Context) -> Result<String, tera::Error> {
    let mut context = Context::from(config_context.clone());
    context.insert("path", "blog/");
    context.insert("og_type", "website");
    let rendered = TEMPLATES.render("blog_list.html", &context)?;
    Ok(rendered)
}

/** renders home page */
pub fn render_home(config_context: &Context) -> Result<String, tera::Error> {
    let mut context = Context::from(config_context.clone());
    context.insert("path", "");
    context.insert("og_type", "website");

    let rendered = TEMPLATES.render("home.html", &context)?;
    Ok(rendered)
}

/** renders given content and metadata into html template */
pub fn render_html(
    content: &str,
    metadata: HashMap<String, String>,
    config_context: &Context,
    slug: &str,
) -> Result<String, tera::Error> {
    let mut context = Context::from(config_context.clone());
    // insert the frontmatter metadata
    context.extend(Context::from_serialize(metadata)?);
    context.insert("content", content);
    context.insert("path", &format!("blog/{}", slug));
    context.insert("og_type", "article");

    let rendered = TEMPLATES.render("blog.html", &context)?;
    Ok(rendered)
}

pub fn render_sitemap(config_context: &Context) -> Result<String, tera::Error> {
    let blogs = match config_context.get("blogs") {
        Some(blogs) => blogs,
        None => return Ok("".to_string()),
    };
    // filter out external blogs
    let mut native_blogs = Vec::new();
    for b in blogs.as_array().unwrap() {
        if b.get("platform").map_or(true, |v| v.is_null()) {
            native_blogs.push(b);
        }
    }
    let mut context = Context::from(config_context.clone());
    context.insert("blogs", &native_blogs);
    let rendered = TEMPLATES.render("sitemap.xml", &context)?;
    Ok(rendered)
}
