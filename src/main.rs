use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Tag name or 'add' command
    command_or_tag: String,

    /// For 'add' command: filename, otherwise: ignored
    #[arg(required = false)]
    file: Option<String>,

    /// Tag name for 'add' command
    #[arg(required = false)]
    tag: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct TagDatabase {
    tags: HashMap<String, Vec<String>>,
}

impl TagDatabase {
    fn load() -> Result<Self> {
        let path = Path::new(".tagfind.json");
        if path.exists() {
            let content = fs::read_to_string(path)
                .context("Failed to read tag database")?;
            serde_json::from_str(&content).context("Failed to parse JSON")
        } else {
            Ok(Self::default())
        }
    }

    fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("Failed to convert to JSON")?;
        fs::write(".tagfind.json", content)
            .context("Failed to save tag database")
    }

    fn add_tag(&mut self, tag: String, file: String) {
        self.tags
            .entry(tag)
            .or_default()
            .push(file);
    }

    fn find_by_tag(&self, tag: &str) -> Option<&Vec<String>> {
        self.tags.get(tag)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut db = TagDatabase::load()?;

    if cli.command_or_tag == "add" {
        if let (Some(file), Some(tag)) = (cli.file, cli.tag) {
            db.add_tag(tag.clone(), file.clone());
            db.save()?;
            println!("Added tag '{}' to file '{}'", tag, file);
        } else {
            println!("Usage: tagfind add <filename> <tag>");
        }
    } else {
        // Tag search
        let tag = cli.command_or_tag;
        if let Some(files) = db.find_by_tag(&tag) {
            println!("Files with tag '{}':", tag);
            for file in files {
                println!("  {}", file);
            }
        } else {
            println!("No files found with tag '{}'", tag);
        }
    }

    Ok(())
}
