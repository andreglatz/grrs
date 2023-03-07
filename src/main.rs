use anyhow::{Context, Result};
use clap::Parser;


#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<()> {
    let args = CLI::parse();

    let path = args.path.to_string_lossy();    
    let content = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file: `{}`", path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }

    Ok(())
}
