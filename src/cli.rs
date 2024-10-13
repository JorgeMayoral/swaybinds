use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use directories::BaseDirs;

use crate::keybinds::Keybinds;

#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
pub struct Cli {
    /// Config file path, defaults to $HOME/.config/sway/config
    #[arg(short('c'), long("config"))]
    config_file: Option<PathBuf>,
    /// Search key bindings with the given text in the action
    #[arg(short('s'), long("search"))]
    search: Option<String>,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let path = self
            .config_file
            .clone()
            .unwrap_or(BaseDirs::new().unwrap().config_dir().join("sway/config"));
        let bindings = Keybinds::try_from(path)?;
        let bindings = if let Some(search_term) = self.search {
            bindings.search_action(&search_term)?
        } else {
            bindings
        };
        bindings.draw_table();

        Ok(())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}
