// src/template.rs

use minijinja::{Environment, context, path_loader};
use std::sync::OnceLock;

use crate::{
    config::Config,
    content::{ContentItem, ContentMeta, get_excerpt_html},
};

static ENV: OnceLock<Environment<'static>> = OnceLock::new();

pub(crate) fn render_index_from_loaded(
    config: &Config,
    index_template_name: &str,
    loaded: Vec<&crate::LoadedContent>,
) -> Result<String, minijinja::Error> {
    let env = ENV.get_or_init(|| {
        let mut env = Environment::new();
        env.set_loader(path_loader(&config.site.template_dir));
        env
    });
    let tmpl = env.get_template(index_template_name)?;

    let mut contents: Vec<ContentItem> = loaded
        .iter()
        .map(|lc| {
            let filename = lc
                .output_path
                .strip_prefix(&config.site.output_dir)
                .unwrap_or(&lc.output_path)
                .to_string_lossy()
                .to_string();

            let formatted_date = lc.content.meta.date.format("%B %d, %Y").to_string();
            let excerpt = get_excerpt_html(&lc.content.data, "## Summary");

            ContentItem {
                html: lc.html.clone(),
                meta: lc.content.meta.clone(),
                formatted_date,
                filename,
                content_type: lc.content_type.clone(),
                excerpt,
            }
        })
        .collect();

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
    content_template: &str,
) -> Result<String, minijinja::Error> {
    let env = ENV.get_or_init(|| {
        let mut env = Environment::new();
        env.set_loader(path_loader(&config.site.template_dir));
        env
    });

    let tmpl = env.get_template(content_template)?;

    let context = context! {
        content => html,
        meta => meta,
        config => config
    };

    tmpl.render(context)
}
