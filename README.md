# `unsplash-wallpaper`

A tiny tool to download wallpapers from Unspash API.

## Installing

`cargo install unsplash-wallpaper`

Or build it from source:

`cargo build --release`

## Configuration

Configuration files are taken from:

1. `$XDG_CONFIG_HOME/unsplash-wallpaper` (if `$XDG_CONFIG_HOME` is set)
2. `$HOME/.config/unsplash-wallpaper`

First save your token in plain text format in `<config_dir>/token` (can be obtained [here](https://unsplash.com/developers)). Leading and trailing whitespaces are trimmed.

Then, specify runtime settings in `<config_dir>/params.toml`:

```toml
collections = "1234567"
count = 15
dest_dir = ".local/share/backgrounds"
```

Then by running `unsplash-wallpaper` you'll get 15 images downloaded to `~/.local/share/backgrounds/<N>.jpeg` (where `N` is a monotonically increasing number from 1 to 15).

Files are downloaded in parallel.

For each downloaded file its absolute filepath is printed to stdout, so you can do something like `| head -n1 | set-bg` in your scripts at startup.
