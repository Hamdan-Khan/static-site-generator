# Static Site Generator

A simple static site generator I wrote for my personal site: https://hamdan-khan.github.io

Its built for the purpose of a simple personal site with blogging support. The templates and styling can be customized to liking. Beside the specified fields in the `config.yaml`, new fields can be added and used in the templates like-wise.

`config.yaml` provides most (or maybe all if blogs are not present) of the required context to make the site work.

Blogs, with metadata / SEO, can be written in the `content/` directory using markdown + frontmatter.

## Usage

I use github actions to compile the rust code and release the [binary](github.com/Hamdan-Khan/static-site-generator/releases/latest/).

And in my target repository i.e. where I want to use the SSG, I use github actions workflow (example of [my site's workflow](https://github.com/Hamdan-Khan/Hamdan-Khan.github.io/blob/master/.github/workflows/deploy.yml)) to download the released binary, perform SSG, and deploy the code to github pages.

## Note

The frontmatter parsing lacks quite a lot.
And the blog file discovery is also non-recursive. I might fix this later.
