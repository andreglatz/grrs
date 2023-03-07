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

    let matches = find_matches(&content, &args.pattern);
    for line in matches {
        println!("{}", line)
    }

    Ok(())
}

fn find_matches(content: &str, pattern: &str) -> Vec<String> {
    let mut matches = Vec::new();

    for line in content.lines() {
        if line.contains(pattern) {
            matches.push(line.to_string())
        }
    }

    return matches;
}

#[cfg(test)]
mod tests {
    use crate::find_matches;

    #[test]
    fn find_a_match() {
        let result = find_matches("lorem ipsum\ndolor sit amet", "lorem");
        assert_eq!(result, vec!["lorem ipsum"])
    }
}