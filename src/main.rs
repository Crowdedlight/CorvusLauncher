use std::sync::Arc;
use anyhow::anyhow;
use clap::Parser as _;
use iced::Theme::Dark;
use CorvusLauncher::App;
use CorvusLauncher::Cli;

/// RGBA bytes for the Logo. Generated with `build.rs`
const LOGO: &[u8; 64 * 64 * 4] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.bin"));

fn main() -> anyhow::Result<()> {

    // Parse command line arguments
    let cli = Arc::new(Cli::parse());

    // Setup logging
    CorvusLauncher::logging::initialize(&cli);

    // TODO Parse or create `corvuslauncher.yaml` config file

    //

    // launch app
    iced::application(
            move || {
                App::builder()
                    .cli(Arc::clone(&cli))
                    // .config(Arc::clone(&config))
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

// TODO - On launch, check if settings are set at ~temp location, depending on OS. 
//        If set load and continue, otherwise make window for user to set location for CORE, Modpacks, Arma3 install, and save at temp location. (Like how Hemtt does) 
// TODO - UI show list of modpacks in CORE, Modpacks, Clientside folders. All toggleable with checkbox, and CORE pre-selected
// TODO - Button to start server with selected mods, button to start headless 1 and 2
// TODO - When selected to start, run the functionality the powershell script otherwise did. (Clean keys, go trough mods, look for keys and copy in etc). 
// TODO - If failing to find a key, show error to user in popup
// TODO - launch arma3 server with selected mods passed to it, and disown the process so the launcher can be closed

// TODO - Extra, selection what server-profile to use. Events, or normal. (for netcode settings and log location etc)


// Needs mods struct, and functionality to search in a modfolder for .bikeys