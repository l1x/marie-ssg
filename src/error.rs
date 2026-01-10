// src/error.rs

use std::path::PathBuf;
use thiserror::Error;

use crate::{config::ConfigError, content::ContentError, output::WriteError};

#[derive(Error, Debug)]
pub(crate) enum RunError {
    //
    #[error("Failed to load configuration")]
    Config(#[from] ConfigError),
    //
    #[error("Failed to load content")]
    Content(#[from] ContentError),
    //
    #[error("Failed to render template")]
    Template(#[from] minijinja::Error),
    //
    #[error("Failed to process static files")]
    Static(#[from] StaticError),
    //
    #[error("Failed to write content")]
    Write(#[from] WriteError),
    //
    #[error("{0}")]
    IoError(String),
}

#[derive(Error, Debug)]
pub(crate) enum StaticError {
    #[error("I/O error processing static file {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
