use std::collections::HashMap;
use pulldown_cmark::{Parser, html};

pub struct ParsedContent {
    front_matter: Option<String>,
    content: String
}

// allowed front matter fields
const METADATA: [&str; 5] = ["title", "description", "layout", "date", "featured"];
const REQUIRED_METADATA: [&str; 2] = ["title", "date"];

/**
  splits the markdown content into two parts:
  - metadata / frontmatter
  - actual content
 */
fn split_content(input: &str) -> ParsedContent {
    let trimmed = input.trim_start();
    
    // if it starts with front matter
    if trimmed.starts_with("---\n") || trimmed.starts_with("---\r\n") {
        let rest = &trimmed[3..];
        if let Some(end_pos) = rest.find("\n---\n").or_else(|| rest.find("\r\n---\r\n")) {
            // rest of the frontmatter till the ending ---
            let front_matter = rest[..end_pos].to_string(); 
            // md content after skipping \n---\n
            let content = rest[end_pos + 5..].to_string(); 
            return ParsedContent {
                front_matter: Some(front_matter),
                content,
            };
        }
    }
    
    ParsedContent {
        front_matter: None,
        content: input.to_string(),
    }
}

/**
  a really basic front matter parser to get things going
  
  doesn't parse strings in array, boolean, single / double quote for now
 */
fn parse_front_matter(raw_fm: &str, fm: &mut HashMap<String, String>) -> Result<(), String> {
    for line in raw_fm.lines() {
        if let Some((key, val)) = line.split_once(':') {
            let key = key.trim();
            let val = val.trim();
            
            if !key.is_empty() && !val.is_empty() && METADATA.contains(&key) {
                fm.insert(key.to_string(), val.to_string());
            }
        }
    }

    // validate required fields
    for field in REQUIRED_METADATA {
        if !fm.contains_key(field) {
            return Err(format!("Missing required field: {}", field));
        }
    }
    
    Ok(())
}

pub fn parse_content(md_content: &str) -> Result<(String, HashMap<String, String>), String> {
    let parsed = split_content(md_content);
    let mut front_matter: HashMap<String, String> = HashMap::new();
    
    // handle front matter parsing
    if let Some(fm) = parsed.front_matter {
        parse_front_matter(&fm, &mut front_matter)?;
    }
    
    // convert markdown to HTML
    let mut html_output = String::new();
    let parser = Parser::new(&parsed.content);
    html::push_html(&mut html_output, parser);
    
    Ok((html_output, front_matter))
}