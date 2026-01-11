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

/** renders home page */
pub fn render_index() -> Result<String, tera::Error>{
    let mut context = Context::new();
    context.insert("title", "Hamdan Khan");
    context.insert("name", "Hamdan Khan");
    context.insert("email", "hamdankhan@gmail.com");
    context.insert("bio", "Software Engineer");

    
    let rendered = TEMPLATES.render("base.html", &context)?;
    Ok(rendered)
}

pub fn render_html(content: &str, metadata: HashMap<String,String>) -> Result<String, tera::Error>{
    let mut context = Context::new();
    // makes the frontmatter metadata available in context
    // todo: context managed individually for every file rn, maybe extract common context? 
    context.extend(Context::from_serialize(metadata)?);
    context.insert("content", content);
    context.insert("name", "Hamdan Khan");
    context.insert("email", "hamdankhan@gmail.com");
    context.insert("bio", "Software Engineer");

    
    let rendered = TEMPLATES.render("blog.html", &context)?;
    Ok(rendered)
}