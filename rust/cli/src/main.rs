#![forbid(unsafe_code)]

mod aggr;
mod cli;
mod config;
mod engine;
mod field_help;
mod format;
mod mcp;
mod meta;
mod tpl;
mod writers;

use clap::Parser;

fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();

    // Intercept --version to append fingerprint
    if raw_args.first().map(String::as_str) == Some("--version")
        || raw_args.first().map(String::as_str) == Some("-V")
    {
        println!("seedfaker {} ({})", env!("CARGO_PKG_VERSION"), seedfaker_core::fingerprint());
        return;
    }

    // Context-aware --help: resolve to field-level help when a field name is present
    if raw_args.iter().any(|a| a == "--help" || a == "-h") {
        let context: Vec<&str> =
            raw_args.iter().filter(|a| !a.starts_with('-')).map(String::as_str).collect();
        match context.first() {
            Some(name) if seedfaker_core::field::lookup(name).is_some() => {
                field_help::print_field_help(name);
                return;
            }
            _ => {} // fall through to clap's default --help
        }
    }

    let cli = cli::Cli::parse();
    if let Err(e) = cli::run(cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
