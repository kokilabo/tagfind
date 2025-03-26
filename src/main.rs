use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Command: 'add', 'remove', or tag name to search
    command_or_tag: String,

    /// Filename for 'add'/'remove' commands
    #[arg(required = false)]
    file: Option<String>,

    /// Tag name for 'add'/'remove' commands
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

    fn validate_input(tag: &str, file: &str) -> Result<()> {
        if tag.trim().is_empty() {
            return Err(anyhow!("Tag cannot be empty"));
        }
        if file.trim().is_empty() {
            return Err(anyhow!("File name cannot be empty"));
        }
        if !Path::new(file).exists() {
            return Err(anyhow!("File '{}' does not exist", file));
        }
        Ok(())
    }

    fn add_tag(&mut self, tag: String, file: String) -> Result<bool> {
        Self::validate_input(&tag, &file)?;
        let files = self.tags.entry(tag.clone()).or_default();
        if files.contains(&file) {
            Ok(false) // Already tagged
        } else {
            files.push(file);
            Ok(true)
        }
    }

    fn remove_tag(&mut self, tag: &str, file: &str) -> Result<bool> {
        Self::validate_input(tag, file)?;
        if let Some(files) = self.tags.get_mut(tag) {
            if let Some(pos) = files.iter().position(|f| f == file) {
                files.remove(pos);
                if files.is_empty() {
                    self.tags.remove(tag);
                }
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn find_by_tag(&self, tag: &str) -> Option<&Vec<String>> {
        if tag.trim().is_empty() {
            None
        } else {
            self.tags.get(tag)
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut db = TagDatabase::load()?;

    match cli.command_or_tag.as_str() {
        "add" => {
            if let (Some(file), Some(tag)) = (cli.file, cli.tag) {
                match db.add_tag(tag.clone(), file.clone()) {
                    Ok(true) => {
                        db.save()?;
                        println!("Added tag '{}' to file '{}'", tag, file);
                    }
                    Ok(false) => {
                        println!("File '{}' is already tagged with '{}'", file, tag);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            } else {
                println!("Usage: tagfind add <filename> <tag>");
            }
        }
        "remove" => {
            if let (Some(file), Some(tag)) = (cli.file, cli.tag) {
                match db.remove_tag(&tag, &file) {
                    Ok(true) => {
                        db.save()?;
                        println!("Removed tag '{}' from file '{}'", tag, file);
                    }
                    Ok(false) => {
                        println!("File '{}' is not tagged with '{}'", file, tag);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            } else {
                println!("Usage: tagfind remove <filename> <tag>");
            }
        }
        tag => {
            if let Some(files) = db.find_by_tag(tag) {
                if files.is_empty() {
                    println!("No files found with tag '{}'", tag);
                } else {
                    println!("Files with tag '{}':", tag);
                    for file in files {
                        println!("  {}", file);
                    }
                }
            } else {
                println!("No files found with tag '{}'", tag);
            }
        }
    }

    Ok(())
}
