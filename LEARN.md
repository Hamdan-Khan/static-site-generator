# How the Static Site Generator Works

This static site generator is written in Rust and converts your Markdown entries and configurations into a complete, static HTML website ready to be deployed.

## 1. Flow of the Generator

When the generator executes, it follows these steps:

1. **Reads Content**: Scans the `content/` directory for any Markdown (`.md`) files.
2. **Parses Configuration & Markdown**: Reads `config.yaml` to build the required global context (like your profile, social links, experiences). Then, it parses the frontmatter and markdown body of each blog post.
3. **Prepares Output**: Creates a fresh `public/` build directory.
4. **Renders Templates**: Injects the parsed metadata and content into the [Tera](https://keats.github.io/tera/) templates located in the `templates/` directory.
5. **Generates Files**:
   - `public/index.html` (Home page)
   - `public/sitemap.xml`
   - `public/blog/index.html` (Blog listing page, if blogs are present)
   - `public/blog/{slug}/index.html` (Individual article pages)
6. **Copies Assets**: Copies all unprocessed files from the `static/` directory directly into the `public/` build directory (e.g., CSS, images).

## 2. Setup & Usage

### Running Locally

Ensure you have Rust and Cargo installed, then execute:

```bash
cargo run
```

You can also compile it for production via `cargo build --release` or download a pre-compiled [binary](https://github.com/Hamdan-Khan/static-site-generator/releases/latest/) for CI/CD deployments.

### Adding & Customizing Templates

The generator utilizes the **Tera** templating engine, which is heavily inspired by Jinja2 and Django templates.

- **Location**: Store your templates in the `templates/` directory (e.g., `base.html`, `home.html`, `blog.html`).
- **Variables**: Any root key in `config.yaml` (e.g., `projects`, `bio`) becomes available in your Tera templates as `{{ projects }}` or `{{ bio }}`.
- **Customizing**: You can add new Tera-compatible HTML files to `templates/` and extend them from `base.html` using `{% extends \"base.html\" %}`.

### Expanding Content

- **Global Context**: Modify `config.yaml` to update standard profile information, external blog links, side projects, and basic configurations.
- **Blogs**: Create new Markdown files directly inside the `content/` folder (note: recursive sub-directory checking is not supported yet). Metadata should be defined in standard frontmatter block at the top of the file.
- **Static files**: Add your CSS styles, JS scripts, or images to the `static/` folder so they get automatically shipped to the `public/` build.
