use clap::Parser;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLI::parse();
    let content = std::fs::read_to_string(&args.path)?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }

    Ok(())
}
