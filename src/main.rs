mod app;
pub mod components;
pub mod helpers;
pub mod pages;
mod tabs;
pub mod traits;
use std::path::PathBuf;

use app::App;
use clap::{command, Parser};
use color_eyre::Result;
use helpers::config::{Config, ReplaceColors};
use pages::image_input::ImageInputTui;
use ratatui::style::Color;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// A program that extract colors from image and applies them to files
struct Args {
    /// Custom path to config file
    #[arg(short, long)]
    path_cfg: Option<PathBuf>,
    /// Enables cli mode
    #[arg(short, long, default_missing_value = "true", default_value = "false")]
    cli: bool,
    /// Path to image that will be used in cli mode or opened in tui
    #[arg(short, long)]
    image: Option<PathBuf>,
    /// Silence all output in cli mode
    #[arg(short, long, default_missing_value = "true", default_value = "false")]
    silence: bool,
}

pub struct Logger {
    pub silent: bool,
}
impl Logger {
    pub fn log(&self, data: &str) {
        if !self.silent {
            println!("{}", data)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let cfg = if let Some(path) = args.path_cfg {
        Config::from_path(path)
    } else {
        Config::new()
    };
    if args.cli {
        let logger = Logger {
            silent: args.silence,
        };
        logger.log("Getting colors from image...");
        let image = args
            .image
            .expect("--image parameter shoudn't be empty!")
            .into_os_string()
            .into_string()
            .expect("Failed to use image path");
        let colors = image_palette::load(&image).expect("Failed to extract colors from image");
        let colors = colors.0.iter().map(|x| {
            let color = x.color();
            Color::Rgb(color.0, color.1, color.2).to_string()
        }).collect::<Vec<String>>();
        let colors = ReplaceColors {
            primary: colors[0].clone(),
            secondary: colors[1].clone(),
            tertiary: colors[2].clone(),
        };
        logger.log(&format!("Got colors from image {:?}", colors));
        logger.log("Replacing files...");
        cfg.process(&colors);
        logger.log("Completed!");
        Ok(())
    } else {
        color_eyre::install()?;
        let mut terminal = ratatui::init();
        terminal.clear().unwrap();
        let mut app = App::new(cfg);
        if let Some(path) = args.image {
            app.tx
                .send(
                    ImageInputTui::UsePath(path.into_os_string().into_string().unwrap()).into(),
                )
                .await
                .expect("Failed to use image path");
        }
        app.run(terminal).await.unwrap();
        ratatui::restore();
        Ok(())
    }
}
