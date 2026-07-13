use clap::Parser;

/// Termino — A sleek terminal-based timer and stopwatch CLI tool.
fn main() -> anyhow::Result<()> {
    let args = termino::cli::Args::parse();
    termino::cli::run(args)?;
    Ok(())
}