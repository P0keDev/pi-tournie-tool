#![feature(try_blocks)]

use disk::SDHandler;

use crate::app::App;

pub mod app;
pub mod event;
pub mod gpio;
pub mod ui;
pub mod tabs;
pub mod disk;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
