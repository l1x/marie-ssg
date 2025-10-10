// src/template.rs

use minijinja::{Environment, context, path_loader};
use std::path::PathBuf;

use crate::{
    config::Config,
    content::{ContentItem, ContentMeta, convert_content, get_excerpt_html, load_content},
    utils::{add_date_prefix, get_content_type, get_output_path},
};

pub(crate) fn render_index_with_contents(
    config: &Config,
    index_template_name: &str,
    content_files: Vec<&PathBuf>,
) -> Result<String, minijinja::Error> {
    let mut env = Environment::new();
    env.set_loader(path_loader(&config.template_dir));
    let tmpl = env.get_template(index_template_name)?;

    let mut contents = Vec::new();
    for file in content_files {
        let content_type = get_content_type(file, &config.content_dir);

        // Load the content
        let content = load_content(file).map_err(|e| {
            minijinja::Error::new(
                minijinja::ErrorKind::InvalidOperation,
                format!("Failed to load content: {}", e),
            )
        })?;

        // Convert the markdown to HTML
        let html = convert_content(&content, file.clone()).map_err(|e| {
            minijinja::Error::new(
                minijinja::ErrorKind::InvalidOperation,
                format!("Failed to convert content: {}", e),
            )
        })?;

        let mut output_path = get_output_path(file, &config.content_dir, &config.output_dir);

        // Apply date prefix if configured for this content type
        if let Some(content_type_config) = config.content_types.get(&content_type) {
            if content_type_config.output_naming.as_deref() == Some("date") {
                output_path = add_date_prefix(output_path, &content.meta.date);
            }
        }

        let filename = output_path
            .strip_prefix(&config.output_dir)
            .unwrap_or(&output_path)
            .to_string_lossy()
            .to_string();

        let formatted_date = content.meta.date.format("%B %d, %Y").to_string();

        // Extract excerpt from the markdown content
        let excerpt = get_excerpt_html(&content.data, "## Summary");

        contents.push(ContentItem {
            html,
            meta: content.meta,
            formatted_date,
            filename,
            content_type,
            excerpt,
        });
    }

    contents.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));

    let context = context! {
        config => config,
        contents => contents,
    };

    tmpl.render(context)
}

pub(crate) fn render_html(
    html: &str,
    meta: &ContentMeta,
    config: &Config,
    template_dir: &str,
    content_template: &str,
) -> Result<String, minijinja::Error> {
    let mut env = Environment::new();
    env.set_loader(path_loader(template_dir));

    let tmpl = env.get_template(content_template)?;

    let context = context! {
        content => html,
        meta => meta,
        config => config
    };

    tmpl.render(context)
}

pub(crate) fn get_content_by_type<'a>(
    files: &'a [PathBuf],
    content_type: &'a str,
) -> Vec<&'a PathBuf> {
    files
        .into_iter()
        .filter(|f| f.components().any(|x| x.as_os_str() == content_type))
        .collect()
}
