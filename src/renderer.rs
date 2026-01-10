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

pub fn render_html(content: &str, title: String) -> Result<String, tera::Error>{
    let mut context = Context::new();
    context.insert("title", title.as_str());
    context.insert("content", content);

    let rendered = TEMPLATES.render("base.html", &context)?;
    Ok(rendered)
}