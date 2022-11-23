use std::path::Path;
use iced::{Application, Settings};
use m4::m4::M4;
use m4::mod_conf::ModConfig;
use m4::util::load_file_into;

fn main() -> iced::Result {
    let mut settings: Settings<_> = Settings::default();
    settings.window.min_size = Some((460, 320));
    M4::run(settings.into())
}
