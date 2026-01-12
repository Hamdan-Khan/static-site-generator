use std::collections::HashMap;
use tera::{Tera, Context};
use lazy_static::lazy_static;

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

pub fn render_blogs_list(config_context: &Context) -> Result<String, tera::Error>{
    let mut context = Context::from(config_context.clone());
    context.insert("title", "Hamdan Khan");
    let rendered = TEMPLATES.render("blog_list.html", &context)?;
    Ok(rendered)
}

/** renders home page */
pub fn render_home(config_context: &Context) -> Result<String, tera::Error>{
    let mut context = Context::from(config_context.clone());
    context.insert("title", "Hamdan Khan");

    let rendered = TEMPLATES.render("base.html", &context)?;
    Ok(rendered)
}

/** renders given content and metadata into html template */
pub fn render_html(content: &str, metadata: HashMap<String,String>, config_context: &Context) -> Result<String, tera::Error>{
    let mut context = Context::from(config_context.clone());
    // insert the frontmatter metadata
    context.extend(Context::from_serialize(metadata)?);
    context.insert("content", content);

    let rendered = TEMPLATES.render("blog.html", &context)?;
    Ok(rendered)
}
