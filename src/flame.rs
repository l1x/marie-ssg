// src/flame.rs

use std::fs::File;
use std::io::BufWriter;
use tracing::info;
use tracing_flame::FlameLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::build::build_with_spans;
use crate::error::RunError;

/// Build the site with profiling and generate output in requested formats.
///
/// # Arguments
/// * `config_file` - Path to the site configuration file
/// * `output_base` - Base path for output files (extensions added based on flags)
/// * `fold` - Output folded stacks file (.folded) for speedscope/inferno
/// * `svg` - Output SVG flamegraph (.svg)
/// * `time` - Output Chrome DevTools JSON (.json) for timeline view
///
/// If no format flags are specified, defaults to SVG output only.
pub(crate) fn flame(
    config_file: &str,
    output_base: &str,
    fold: bool,
    svg: bool,
    time: bool,
) -> Result<(), RunError> {
    // Default to SVG if no flags specified
    let (fold, svg, time) = if !fold && !svg && !time {
        (false, true, false)
    } else {
        (fold, svg, time)
    };

    // Chrome DevTools JSON format (timeline with timestamps)
    if time {
        return flame_chrome(config_file, output_base);
    }

    // Folded stacks format (with optional SVG generation)
    flame_folded(config_file, output_base, fold, svg)
}

/// Build with tracing-chrome layer to output Chrome DevTools JSON.
fn flame_chrome(config_file: &str, output_base: &str) -> Result<(), RunError> {
    use tracing_chrome::ChromeLayerBuilder;

    let json_path = format!("{}.json", output_base);

    let (chrome_layer, guard) = ChromeLayerBuilder::new()
        .file(&json_path)
        .include_args(true)
        .build();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "marie_ssg=trace".into()),
        )
        .with(chrome_layer)
        .init();

    info!("flame::start profiling build (chrome timeline)");

    // Run the build with detailed spans
    build_with_spans(config_file)?;

    // Flush the chrome layer
    drop(guard);

    info!("flame::complete → {}", json_path);

    Ok(())
}

/// Build with tracing-flame layer to output folded stacks and optionally SVG.
fn flame_folded(
    config_file: &str,
    output_base: &str,
    keep_folded: bool,
    generate_svg: bool,
) -> Result<(), RunError> {
    let folded_path = format!("{}.folded", output_base);
    let svg_path = format!("{}.svg", output_base);

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

    info!("flame::start profiling build (folded stacks)");

    // Run the build with detailed spans
    build_with_spans(config_file)?;

    // Flush the flame layer
    drop(guard);

    // Generate SVG if requested
    if generate_svg {
        info!("flame::generate svg → {}", svg_path);
        generate_flamechart(&folded_path, &svg_path)?;
    }

    // Clean up folded file unless --fold was specified
    if !keep_folded {
        std::fs::remove_file(&folded_path).ok();
    } else {
        info!("flame::folded → {}", folded_path);
    }

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
