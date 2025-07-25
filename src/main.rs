mod app;
mod tabs;
pub mod traits;
pub mod components;
pub mod pages;
pub mod helpers;
use color_eyre::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    App::new().run(terminal).await.unwrap();
    ratatui::restore();
    Ok(())
}

