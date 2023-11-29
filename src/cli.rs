use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(name = "epicshot")]
#[command(author = "Nirlep5252 <nirlep5252@proton.me>")]
#[command(about = "A simple screenshot tool written in Rust")]
#[command(author, version, about, long_about=None)]
pub(crate) struct Cli {
    /// Take screenshot of all screens
    #[clap(long)]
    pub(crate) all: bool,

    /// Take screenshot of a specific monitor
    #[clap(long)]
    pub(crate) monitor: Option<usize>,

    /// Take screenshot of a specific window
    #[clap(long)]
    pub(crate) window: Option<String>,

    /// Take screenshot of a selection
    #[clap(long)]
    pub(crate) selection: Option<ScreenshotSelection>,

    /// If you are using X11
    #[clap(long)]
    pub(crate) x11: bool,

    /// If you are using Wayland
    #[clap(long)]
    pub(crate) wayland: bool,

    /// Copy screenshot to clipboard
    #[clap(long)]
    pub(crate) clipboard: bool,

    /// Save screenshot to a file
    #[clap(long)]
    pub(crate) save: Option<String>,
}

#[derive(Debug, Parser, Clone, Copy)]
pub(crate) struct ScreenshotSelection {
    pub(crate) x: i16,
    pub(crate) y: i16,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl FromStr for ScreenshotSelection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_whitespace().collect::<Vec<&str>>();
        if s.len() != 4 {
            Err("Invalid selection input. It should be `<x> <y> <width> <height>`")
        } else {
            let x = s[0]
                .parse::<i16>()
                .map_err(|_| "invalid `x` value, it should be of type `i16`")?;
            let y = s[1]
                .parse::<i16>()
                .map_err(|_| "invalid `y` value, it should be of type `i16`")?;
            let width = s[2]
                .parse::<u16>()
                .map_err(|_| "invalid `width` value, it should be of type `u16`")?;
            let height = s[3]
                .parse::<u16>()
                .map_err(|_| "invalid `height` value, it should be of type `u16`")?;
            Ok(Self {
                x,
                y,
                width,
                height,
            })
        }
    }
}
