use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "epicshot")]
#[command(author = "Nirlep5252 <nirlep5252@proton.me>")]
#[command(about = "A simple screenshot tool written in Rust")]
#[command(author, version, about, long_about=None)]
pub(crate) struct Cli {
    /// Take screenshot of all screens
    #[clap(long)]
    pub(crate) all: bool,

    /// Take screenshot of a specific screen
    #[clap(long)]
    pub(crate) screen: Option<u32>,

    /// Take screenshot of a specific window
    #[clap(long)]
    pub(crate) window: Option<u32>,

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
