use std::{error, fmt};

use crate::{
    config::{APPLICATION_ID, APPLICATION_TITLE, Config, ConfigError},
    theme::ThemeName,
    tuning::{NoteFormat, Tuning},
    widget::Fretboard,
};

const DEFAULT_PADDING: iced::Pixels = iced::Pixels(10.0);

pub fn run() -> Result<(), AppError> {
    let mut window_settings = iced::window::Settings::default();
    window_settings.platform_specific.application_id = String::from(APPLICATION_ID);
    let app = iced::application(boot, update, view)
        .window(window_settings)
        .title(|state: &State| match state {
            State::Running(data) => match &data.tuning.selected {
                Some(tuning) => format!("{APPLICATION_TITLE} - {tuning}"),
                None => String::from(APPLICATION_TITLE),
            },
            State::ConfigError(_) => format!("{APPLICATION_TITLE} - Configuration Error"),
        })
        .theme(|state: &State| {
            iced::Theme::from(match state {
                State::Running(data) => data.theme_name,
                State::ConfigError(_) => ThemeName::default(),
            })
        });
    app.run()?;
    Ok(())
}

#[derive(Debug)]
pub enum AppError {
    Ui(iced::Error),
}

impl From<iced::Error> for AppError {
    fn from(value: iced::Error) -> Self {
        Self::Ui(value)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ui(err) => err.fmt(out),
        }
    }
}

impl error::Error for AppError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Self::Ui(err) => err,
        })
    }
}

#[derive(Debug)]
enum State {
    Running(StateData),
    ConfigError(ConfigError),
}

#[derive(Debug)]
struct StateData {
    note_format: NoteFormat,
    theme_name: ThemeName,
    tuning: StateTuning,
}

#[derive(Debug)]
struct StateTuning {
    combo_box: iced::widget::combo_box::State<Tuning>,
    selected: Option<Tuning>,
}

impl StateData {
    fn new(config: Config) -> Self {
        let tuning_selected = config.tuning.get_selected().clone();
        let tuning = config.tuning.items.clone();
        Self {
            note_format: config.note_format,
            theme_name: config.theme_name,
            tuning: StateTuning {
                combo_box: iced::widget::combo_box::State::new(tuning),
                selected: Some(tuning_selected),
            },
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    NoteFormatSelected(NoteFormat),
    TuningSelected(Tuning),
}

fn boot() -> State {
    match Config::read_from_file() {
        Ok(config) => State::Running(StateData::new(config)),
        Err(err) => {
            eprintln!("{:?}", err);
            State::ConfigError(err)
        }
    }
}

fn update(state: &mut State, message: Message) {
    let State::Running(state_data) = state else {
        return;
    };
    match message {
        Message::NoteFormatSelected(note_format) => state_data.note_format = note_format,
        Message::TuningSelected(tuning) => state_data.tuning.selected = Some(tuning),
    }
}

fn view(state: &State) -> iced::Element<'_, Message> {
    match state {
        State::Running(data) => view_running(data),
        State::ConfigError(err) => view_config_error(err),
    }
}

fn view_running(data: &StateData) -> iced::Element<'_, Message> {
    let tuning_selected = &data.tuning.selected;
    let note_format_selected = Some(data.note_format);
    let fretboard: iced::Element<Message> = match tuning_selected {
        Some(tuning) => Fretboard::new(tuning.clone(), data.note_format, data.theme_name).into(),
        None => iced::widget::text!("Select tuning").into(),
    };
    iced::widget::container(
        iced::widget::column![
            iced::widget::container(fretboard).width(iced::Length::FillPortion(3)),
            iced::widget::row![
                iced::widget::container(iced::widget::combo_box(
                    &data.tuning.combo_box,
                    "Tuning",
                    tuning_selected.as_ref(),
                    Message::TuningSelected
                ))
                .width(iced::Length::FillPortion(3)),
                iced::widget::radio(
                    "Flat",
                    NoteFormat::Flat,
                    note_format_selected,
                    Message::NoteFormatSelected
                ),
                iced::widget::radio(
                    "Sharp",
                    NoteFormat::Sharp,
                    note_format_selected,
                    Message::NoteFormatSelected
                )
            ]
            .spacing(DEFAULT_PADDING)
            .align_y(iced::alignment::Vertical::Center),
        ]
        .spacing(DEFAULT_PADDING),
    )
    .padding(iced::padding::all(DEFAULT_PADDING))
    .into()
}

fn view_config_error(err: &ConfigError) -> iced::Element<'_, Message> {
    let message = err.to_string();
    iced::widget::text(message).into()
}
