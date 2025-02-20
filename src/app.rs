use std::{error, fmt};

use crate::{
    config::{Config, ConfigError, APPLICATION_ID, APPLICATION_TITLE},
    theme::ThemeName,
    tuning::{NoteFormat, Tuning},
    widget::Fretboard,
};

const DEFAULT_PADDING: u16 = 10;

pub fn run() -> Result<(), AppError> {
    let mut window_settings = iced::window::Settings::default();
    window_settings.platform_specific.application_id = String::from(APPLICATION_ID);
    let app = iced::application(APPLICATION_TITLE, update, view)
        .window(window_settings)
        .theme(|state| iced::Theme::from(state.theme_name));
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
    note_format: NoteFormat,
    theme_name: ThemeName,
    tuning: StateTuning,
}

#[derive(Debug)]
struct StateTuning {
    combo_box: iced::widget::combo_box::State<Tuning>,
    selected: Option<Tuning>,
}

impl State {
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

fn update(state: &mut State, message: Message) {
    match message {
        Message::NoteFormatSelected(note_format) => state.note_format = note_format,
        Message::TuningSelected(tuning) => state.tuning.selected = Some(tuning),
    }
}

fn view(state: &State) -> iced::Element<Message> {
    let tuning_selected = &state.tuning.selected;
    let note_format_selected = Some(state.note_format);
    let fretboard: iced::Element<Message> = match tuning_selected {
        Some(tuning) => Fretboard::new(tuning.clone(), state.note_format, state.theme_name).into(),
        None => iced::widget::text!("Select tuning").into(),
    };
    iced::widget::container(
        iced::widget::column![
            iced::widget::container(fretboard).width(iced::Length::FillPortion(3)),
            iced::widget::row![
                iced::widget::container(iced::widget::combo_box(
                    &state.tuning.combo_box,
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
    .padding([DEFAULT_PADDING, DEFAULT_PADDING])
    .into()
}
