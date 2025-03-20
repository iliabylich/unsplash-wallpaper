use anyhow::{Context as _, Result, bail};

pub(crate) struct Dirs;

impl Dirs {
    pub(crate) fn configs_dir() -> Result<String> {
        let user_config_dir = if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
            xdg_config_home
        } else if let Ok(home) = std::env::var("HOME") {
            format!("{home}/.config")
        } else {
            bail!("Both $XDG_CONFIG_HOME and $HOME are not set")
        };
        Ok(format!("{}/unsplash-wallpaper", user_config_dir))
    }

    pub(crate) fn home_dir() -> Result<String> {
        std::env::var("HOME").context("$HOME is not set")
    }
}
