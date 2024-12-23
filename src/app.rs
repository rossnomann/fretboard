use std::{error, fmt};

use crate::{
    config::{Config, ConfigError, APPLICATION_ID, APPLICATION_TITLE},
    widget::Fretboard,
};

pub fn run() -> Result<(), AppError> {
    let mut window_settings = iced::window::Settings::default();
    window_settings.platform_specific.application_id = String::from(APPLICATION_ID);
    let app = iced::application(APPLICATION_TITLE, update, view)
        .window(window_settings)
        .theme(|state| iced::Theme::from(state.config.theme_name));
    let config = Config::read_from_file()?;
    app.run_with(move || (State::new(config), iced::Task::none()))?;
    Ok(())
}

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Ui(iced::Error),
}

impl From<ConfigError> for AppError {
    fn from(value: ConfigError) -> Self {
        Self::Config(value)
    }
}

impl From<iced::Error> for AppError {
    fn from(value: iced::Error) -> Self {
        Self::Ui(value)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Config(err) => err.fmt(out),
            Self::Ui(err) => err.fmt(out),
        }
    }
}

impl error::Error for AppError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Self::Config(err) => err,
            Self::Ui(err) => err,
        })
    }
}

#[derive(Debug)]
struct State {
    config: Config,
}

impl State {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Debug)]
enum Message {}

fn update(state: &mut State, message: Message) {
    println!("STATE: {:?}; MESSAGE: {:?}", state, message)
}

fn view(state: &State) -> iced::Element<Message> {
    iced::widget::row![
        Fretboard::new(state.config.tuning.get_selected().clone(), state.config.theme_name),
        iced::widget::text("TODO: CONFIG")
    ]
    .into()
}
