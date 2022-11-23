use std::path::{Path, PathBuf};
use iced::{Application, Column, Command, Container, Element, executor, Length, Text};
use crate::{constants, fts, util};
use crate::fts::FirstTimeSetup;
use crate::m4_settings::{LoadError, LoadErrorType, M4Settings};
use crate::util::{is_valid_megamix_dir, load_file_into};

#[derive(PartialEq)]
pub enum M4State{
    // Shows a loading screen
    LoadingScreen,
    // Shows an error screen
    ErrorScreen(String),
    // If settings cant be loaded, or if they are not set up
    FirstTimeSetup,
    // Main screen
    Home,
}

pub struct M4{
    state: M4State,
    settings: M4Settings,
    fts: FirstTimeSetup,
}

impl M4{
    
}

#[derive(Debug)]
pub enum Message{
    LoadedSettings(Result<M4Settings, LoadError>),
    FTSMessage(fts::FTSMessage),
}

impl Application for M4 {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let state = M4State::LoadingScreen;
        let settings = M4Settings::default();
        (
            M4 {
                state,
                settings,
                fts: FirstTimeSetup::new(),
            },
            // On start up, perform a command to load the settings.
            Command::perform(
                load_file_into::<M4Settings>(util::get_config_path().expect("Failed to get config path")),
                Message::LoadedSettings)
        )
    }

    fn title(&self) -> String {
        String::from(constants::APPNAME)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        return match message {
            Message::LoadedSettings(settings) => {
                if settings.is_err() {
                    //Check error
                    //If error is file not found, then go to first time setup
                    //If error is cant open file, then go to error screen
                    return match settings.err().unwrap().0 {
                        LoadErrorType::CantOpenFile => {
                            self.state = M4State::ErrorScreen(String::from("Cannot open settings file!"));
                            Command::none()
                        }
                        LoadErrorType::FileNotFound => {
                            //If were here that means the settings file doesn't exist,
                            //so the user needs to go though setup.
                            self.state = M4State::FirstTimeSetup;
                            Command::none()
                        }
                    }
                }

                //If were here that means the settings file exists,
                //so lets do a simple check for required settings for home.
                self.settings = settings.unwrap();
                if self.settings.game_dir.is_empty() {
                    //Game dir doesn't exist.
                    self.state = M4State::FirstTimeSetup;
                } else if is_valid_megamix_dir(Path::new(&self.settings.game_dir)) {
                    //The game dir exists, and is valid.
                    self.state = M4State::Home;
                } else {
                    //The game dir exists, but is invalid.
                    self.state = M4State::FirstTimeSetup;
                }
                Command::none()
            }
            Message::FTSMessage(msg) => {
                self.fts.update(msg).map(|cmd| Message::FTSMessage(cmd))
            }

        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content: Element<_> = match &self.state {
            M4State::LoadingScreen =>
                Text::new("Loading...").size(40).into(),

            M4State::ErrorScreen(msg) =>
                Text::new(msg).size(30).into(),

            M4State::Home =>
                Text::new("Home").size(30).into(),

            M4State::FirstTimeSetup =>
                self.fts.view().map(Message::FTSMessage).into(),
        };

        Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}