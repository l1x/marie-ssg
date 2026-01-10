// src/flame.rs

use std::fs::File;
use std::io::BufWriter;
use tracing::info;
use tracing_flame::FlameLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::build::build_with_spans;
use crate::error::RunError;

/// Build the site with profiling and generate a flamechart SVG.
pub(crate) fn flame(config_file: &str, output_path: &str) -> Result<(), RunError> {
    // Create a file for the folded stacks output
    let folded_path = format!("{}.folded", output_path);

    // Set up tracing with flame layer
    let (flame_layer, guard) = FlameLayer::with_file(&folded_path).map_err(|e| {
        RunError::IoError(format!("Failed to create flame layer: {}", e))
    })?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "marie_ssg=trace".into()),
        )
        .with(flame_layer)
        .init();

    info!("flame::start profiling build");

    // Run the build with detailed spans
    build_with_spans(config_file)?;

    // Flush the flame layer
    drop(guard);

    info!("flame::generate → {}", output_path);

    // Generate flamechart SVG from folded stacks
    generate_flamechart(&folded_path, output_path)?;

    // Clean up the intermediate folded file
    std::fs::remove_file(&folded_path).ok();

    info!("flame::complete ✓");

    Ok(())
}

/// Generate flamechart SVG from folded stacks using inferno.
fn generate_flamechart(folded_path: &str, svg_path: &str) -> Result<(), RunError> {
    use inferno::flamegraph::{self, Options};

    let folded_file = File::open(folded_path).map_err(|e| {
        RunError::IoError(format!("Failed to open folded stacks file: {}", e))
    })?;

    let svg_file = File::create(svg_path).map_err(|e| {
        RunError::IoError(format!("Failed to create SVG file: {}", e))
    })?;

    let mut options = Options::default();
    options.title = "Marie SSG Build Profile".to_string();
    options.subtitle = Some("Function call flamechart".to_string());

    let reader = std::io::BufReader::new(folded_file);
    let writer = BufWriter::new(svg_file);

    flamegraph::from_reader(&mut options, reader, writer).map_err(|e| {
        RunError::IoError(format!("Failed to generate flamechart: {}", e))
    })?;

    Ok(())
}
