use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// タグ名または'add'コマンド
    command_or_tag: String,

    /// addコマンドの場合: ファイル名、それ以外の場合: 無視
    #[arg(required = false)]
    file: Option<String>,

    /// addコマンドの場合のタグ名
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
                .context("タグデータベースの読み込みに失敗しました")?;
            serde_json::from_str(&content).context("JSONのパースに失敗しました")
        } else {
            Ok(Self::default())
        }
    }

    fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("JSONへの変換に失敗しました")?;
        fs::write(".tagfind.json", content)
            .context("タグデータベースの保存に失敗しました")
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
            println!("タグ '{}' をファイル '{}' に追加しました", tag, file);
        } else {
            println!("使用方法: tagfind add <ファイル名> <タグ名>");
        }
    } else {
        // タグ検索
        let tag = cli.command_or_tag;
        if let Some(files) = db.find_by_tag(&tag) {
            println!("タグ '{}' が付いているファイル:", tag);
            for file in files {
                println!("  {}", file);
            }
        } else {
            println!("タグ '{}' が付いているファイルは見つかりませんでした", tag);
        }
    }

    Ok(())
}
