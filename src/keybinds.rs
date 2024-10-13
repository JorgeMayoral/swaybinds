use anyhow::{anyhow, Error, Result};
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use std::{fs, path::PathBuf};

pub struct Keybinds(Vec<String>);

impl Keybinds {
    pub fn search_action(self, search_term: &str) -> Result<Self> {
        let bindings = self
            .0
            .iter()
            .filter(|line| line.contains(search_term))
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<String>>();
        if bindings.is_empty() {
            return Err(anyhow!(
                "Can't find key bindings with that text in the action."
            ));
        }
        Ok(Keybinds(bindings))
    }

    pub fn draw_table(self) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Shortcut", "Action"]);
        for line in self.0 {
            let (shortcut, action) = line.split_once(' ').unwrap();
            table.add_row(vec![shortcut, action]);
        }
        println!("{table}");
    }
}

impl TryFrom<PathBuf> for Keybinds {
    type Error = Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let content = fs::read_to_string(value)?;
        let bindings = content
            .lines()
            .map(|line| line.trim().to_owned())
            .filter(|line| line.starts_with("bindsym"))
            .map(|line| line.strip_prefix("bindsym ").unwrap().to_owned())
            .collect::<Vec<String>>();
        if bindings.is_empty() {
            return Err(anyhow!("Can't find key bindings in this file."));
        }
        Ok(Self(bindings))
    }
}
