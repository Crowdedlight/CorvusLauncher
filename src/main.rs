use corvus_launcher::Cli;
use corvus_launcher::{App, Config};
use anyhow::anyhow;
use clap::Parser as _;
use iced::Theme::Dark;
use std::sync::{Arc, RwLock};

/// RGBA bytes for the Logo. Generated with `build.rs`
const LOGO: &[u8; 64 * 64 * 4] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.bin"));

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = Arc::new(Cli::parse());

    // Setup logging
    corvus_launcher::logging::initialize(&cli);

    // init config
    let config = Arc::new(RwLock::new(Config::new()));

    log::info!("Initialized CorvusLauncher");

    // launch app
    iced::application(
        move || {
            App::builder()
                .cli(Arc::clone(&cli))
                .configs(Arc::clone(&config))
                .build()
        },
        App::update,
        App::view,
    )
    // .subscription(App::subscription)
    .window(iced::window::Settings {
        level: iced::window::Level::Normal,
        icon: Some(
            iced::window::icon::from_rgba(LOGO.to_vec(), 64, 64)
                .expect("Icon to be valid RGBA bytes"),
        ),
        ..Default::default()
    })
    .title("CorvusLauncher")
    .theme(Dark)
    .default_font(iced::Font::MONOSPACE)
    .run()
    .map_err(|err| anyhow!("Failed to start CorvusLauncher: {err}"))
}
