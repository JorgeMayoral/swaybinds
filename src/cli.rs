use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use directories::BaseDirs;

#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short('c'), long("config"))]
    config_file: Option<PathBuf>,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let path = self
            .config_file
            .clone()
            .unwrap_or(BaseDirs::new().unwrap().config_dir().join("sway/config"));
        let content = fs::read_to_string(path)?;
        let bindings = content
            .lines()
            .map(|line| line.trim().to_owned())
            .filter(|line| line.starts_with("bindsym"))
            .map(|line| line.strip_prefix("bindsym ").unwrap().to_owned())
            .collect::<Vec<String>>();

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Shortcut", "Action"]);
        for line in bindings {
            let (shortcut, action) = line.split_once(' ').unwrap();
            table.add_row(vec![shortcut, action]);
        }
        println!("{table}");
        Ok(())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}
