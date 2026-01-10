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

pub fn render_html(content: &str, metadata: HashMap<String,String>) -> Result<String, tera::Error>{
    let mut context = Context::new();
    // makes the frontmatter metadata available in context
    context.extend(Context::from_serialize(metadata)?);
    context.insert("content", content);
    // todo: choose template using metadata
    let rendered = TEMPLATES.render("base.html", &context)?;
    Ok(rendered)
}