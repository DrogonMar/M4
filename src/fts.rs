use std::path::Path;
use ashpd::desktop::file_chooser::{FileChooserProxy, OpenFileOptions};
use ashpd::{WindowIdentifier, zbus};
use iced::*;
use iced::button::State;
use iced::svg::Handle;
use url::Url;
use crate::constants;
use crate::style::{M4ButtonStyleSheet, M4InputStyleSheet};
use crate::util::is_valid_megamix_dir;

//FTS, First time setup
//This is a big file, so things are in regions, to make it easier to navigate
//The regions are:
// - First time setup
// - Steps


//This section is the main thing, it makes the navigation and displays the steps
//from the steps section
//region First time setup
#[derive(Debug, Clone)]
pub enum FTSMessage {
    PrevClicked,
    NextClicked,
    FtsStepMsg(StepMessage),

    // Can be used to tell the parent to do something
    FtsDone,
}

pub struct FirstTimeSetup {
    steps: Steps,
    prev_button: button::State,
    next_button: button::State,
}

impl FirstTimeSetup {
    pub fn new() -> Self {
        Self {
            steps: Steps::new(),
            prev_button: button::State::new(),
            next_button: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<FTSMessage> {
        let FirstTimeSetup {
            steps,
            prev_button,
            next_button,
            ..
        } = self;

        let mut controls = Row::new();

        if steps.has_prev() {
            controls = controls.push(
                Button::new(prev_button, Text::new("Previous").size(20))
                    .style(M4ButtonStyleSheet)
                    .on_press(FTSMessage::PrevClicked)
            );
        }

        controls = controls.push(Space::new(Length::Fill, Length::Shrink));

        if steps.can_continue() {
            controls = controls.push(
                Button::new(next_button, Text::new("Next").size(20))
                    .style(M4ButtonStyleSheet)
                    .on_press(FTSMessage::NextClicked)
            );
        }

        let main_content = Container::new(steps.view().map(FTSMessage::FtsStepMsg))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let content: Element<_> = Column::new()
            .push(main_content)
            .push(controls)
            .padding(10)
            .into();

        Container::new(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn update(&mut self, message: FTSMessage) -> Command<FTSMessage> {
        return match message {
            FTSMessage::PrevClicked => {
                self.steps.go_back();
                Command::none()
            }
            FTSMessage::NextClicked => {
                self.steps.advance();
                Command::none()
            }
            FTSMessage::FtsStepMsg(smsg) => self.steps.update(smsg).map(|cmd| FTSMessage::FtsStepMsg(cmd)),
            _ => { Command::none() }
        };
    }
}
//endregion

//region Steps
enum Step {
    Welcome,
    FindGameDirectory {
        text_state: text_input::State,
        browse_btn: button::State,
        dir: String,
    },
    End,
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Self {
        Self {
            steps: vec![
                Step::Welcome,
                Step::FindGameDirectory {
                    text_state: text_input::State::new(),
                    browse_btn: button::State::new(),
                    dir: String::new(),
                },
                Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) -> Command<StepMessage> {
        self.steps[self.current].update(msg)
    }

    fn view(&mut self) -> Element<StepMessage> {
        self.steps[self.current].view()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_prev() {
            self.current -= 1;
        }
    }

    fn has_prev(&self) -> bool {
        self.current > 0
            && self.steps[self.current].can_go_back()
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len()
            && self.steps[self.current].can_continue()
    }
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    GameDirChanged(String),
    BrowseClicked,
}

//Text To Content
//Like
//
// TEXT  -- The h1 header
// SPACING
// content -- The content
const TTC_SPACING: u16 = 3;

impl<'a> Step {
    fn can_continue(&self) -> bool {
        //Thins like the game directory setup should check if the given data is a
        match self {
            Step::FindGameDirectory { dir, .. } => is_valid_megamix_dir(Path::new(dir)),
            Step::End => false,
            _ => true,
        }
    }

    fn can_go_back(&self) -> bool {
        match self {
            Step::Welcome => false,
            _ => true,
        }
    }

    fn view(&mut self) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::FindGameDirectory { text_state, browse_btn, dir } => Self::find_game_directory(text_state, browse_btn, dir),
            Step::End => Self::end()
        }
    }

    fn update(&mut self, msg: StepMessage) -> Command<StepMessage> {
        return match msg {
            StepMessage::GameDirChanged(input) => {
                if let Step::FindGameDirectory { dir, .. } = self {
                    *dir = input;
                }
                Command::none()
            }
            StepMessage::BrowseClicked => {
                Command::perform(Self::browse_for_dir(), StepMessage::GameDirChanged)
            }
        };
    }

    fn welcome() -> Element<'a, StepMessage> {
        Column::new()
            .spacing(15)
            .push(Text::new(format!("Welcome to {}!", constants::APPNAME)).size(42))
            .push(Space::new(Length::Units(TTC_SPACING), Length::Shrink))
            .push(Text::new("We'll need to do some setup before you can continue."))
            .into()
    }

    fn find_game_directory(input_state: &'a mut text_input::State, browse_btn: &'a mut State, dir: &mut String) -> Element<'a, StepMessage> {
        Column::new()
            .spacing(15)
            .padding(40)
            .push(Text::new("Please select your game directory.").size(42))
            .push(Space::new(Length::Units(TTC_SPACING), Length::Shrink))
            .push(
                Row::new()
                    .push(TextInput::new(input_state, "Game directory", dir, |s| StepMessage::GameDirChanged(s))
                        .style(M4InputStyleSheet {
                            show_validity: true,
                            is_valid: is_valid_megamix_dir(Path::new(dir)),
                        })
                        .padding(3).width(Length::FillPortion(2)))
                    .push(Button::new(browse_btn,
                                      Svg::new(
                                          //TODO: Make a cleaner way to do this, like per platform.
                                          Handle::from_path("/usr/share/icons/breeze/actions/16/document-open-folder.svg")
                                      ).width(Length::Units(26)).height(Length::Units(26)),
                    )
                        .style(M4ButtonStyleSheet)
                        .on_press(StepMessage::BrowseClicked))
                    .align_items(Alignment::Center)
            )

            .into()
    }

    fn end() -> Element<'a, StepMessage> {
        Column::new()
            .spacing(15)
            .push(Text::new("Setup complete!"))
            .into()
    }

    async fn browse_for_dir() -> String {
        let connection = zbus::Connection::session().await.unwrap();
        let proxy = FileChooserProxy::new(&connection).await.unwrap();
        let res = proxy.open_file(&WindowIdentifier::None, "Select the Mega Mix directory",
                                  OpenFileOptions::default()
                                      .directory(true)
                                      .accept_label("Select"),
        ).await;
        if res.is_ok() {
            return Url::parse(res.unwrap().uris()[0].clone().as_str()).unwrap().to_file_path().unwrap().to_str().unwrap().to_string();
        }
        String::new()
    }
}
//endregion
