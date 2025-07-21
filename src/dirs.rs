use std::path::PathBuf;

pub(crate) struct Dirs;

impl Dirs {
    pub(crate) fn config_dir() -> PathBuf {
        Self::xdg_config_dir().join("unsplash-wallpaper")
    }

    fn xdg_config_dir() -> PathBuf {
        if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config_home)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config")
        } else {
            log::error!("Both $XDG_CONFIG_HOME and $HOME are not set");
            std::process::exit(1)
        }
    }

    pub(crate) fn home() -> PathBuf {
        let Ok(home) = std::env::var("HOME") else {
            log::error!("$HOME is not set");
            std::process::exit(1)
        };
        PathBuf::from(home)
    }
}
