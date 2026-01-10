// src/main.rs

use argh::FromArgs;
use tracing::error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod asset_hash;
mod build;
mod config;
mod content;
mod error;
mod flame;
mod guide;
mod output;
mod redirect;
mod rss;
mod sitemap;
mod syntax;
mod template;
mod utils;
mod watch;

// Re-export for other modules
pub(crate) use build::LoadedContent;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn default_config_file() -> String {
    "site.toml".to_string()
}

#[derive(FromArgs, Debug)]
/// Marie SSG - Super Simple Static Site Generator
struct Argz {
    /// print version information
    #[argh(switch, short = 'V')]
    version: bool,

    #[argh(subcommand)]
    command: Option<SubCommand>,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommand {
    Build(BuildArgs),
    Watch(WatchArgs),
    Guide(GuideArgs),
    Flame(FlameArgs),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "build")]
/// Build the static site
struct BuildArgs {
    /// path to the config file
    #[argh(option, short = 'c', default = "default_config_file()")]
    config_file: String,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "watch")]
/// Watch for changes and rebuild automatically
struct WatchArgs {
    /// path to the config file
    #[argh(option, short = 'c', default = "default_config_file()")]
    config_file: String,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "guide")]
/// Print a guide explaining Marie SSG features and configuration
struct GuideArgs {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "flame")]
/// Build the site with profiling and generate flamechart output
struct FlameArgs {
    /// path to the config file
    #[argh(option, short = 'c', default = "default_config_file()")]
    config_file: String,

    /// output base path (extensions added based on format flags)
    #[argh(option, short = 'o', default = "default_flame_output()")]
    output: String,

    /// output folded stacks file (.folded) for speedscope/inferno
    #[argh(switch)]
    fold: bool,

    /// output SVG flamegraph (.svg)
    #[argh(switch)]
    svg: bool,

    /// output Chrome DevTools JSON (.json) for timeline view
    #[argh(switch)]
    time: bool,
}

fn default_flame_output() -> String {
    "flamechart".to_string()
}

fn main() {
    // Parse CLI arguments first to check if flame command
    let argz: Argz = argh::from_env();

    if argz.version {
        println!("marie-ssg {}", VERSION);
        return;
    }

    // Flame command has its own tracing setup for profiling
    if let Some(SubCommand::Flame(args)) = argz.command {
        if let Err(e) = flame::flame(&args.config_file, &args.output, args.fold, args.svg, args.time) {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
        return;
    }

    // Initialize standard tracing subscriber for other commands
    // Format: "2025-01-03T12:00:00Z INFO message" (no module path, no spans)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "marie_ssg=info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(tracing_subscriber::fmt::time::UtcTime::new(
                    kiters::timestamp::get_utc_formatter(),
                ))
                .with_target(false) // Remove module path (marie_ssg::output)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE), // Remove span prefixes
        )
        .init();

    match argz.command {
        Some(SubCommand::Build(args)) => {
            if let Err(e) = build::build(&args.config_file) {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
        Some(SubCommand::Watch(args)) => {
            if let Err(e) = watch::watch(&args.config_file) {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
        Some(SubCommand::Guide(_)) => {
            guide::print_guide();
        }
        Some(SubCommand::Flame(_)) => unreachable!(), // Handled above
        None => {
            println!("marie-ssg {}", VERSION);
            println!("Use --help for usage information");
        }
    }
}
