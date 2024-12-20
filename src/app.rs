use crate::{
    config::{Config, APPLICATION_TITLE},
    widget::Fretboard,
};

pub fn run() -> iced::Result {
    let mut window_settings = iced::window::Settings::default();
    window_settings.platform_specific.application_id = String::from("com.github.rossnomann.fretboard");
    let app = iced::application(APPLICATION_TITLE, update, view).window(window_settings);
    let config = Config::default();
    app.run_with(move || (State::new(config), iced::Task::none()))
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
        Fretboard::new(state.config.frets_count, state.config.tuning.clone()),
        iced::widget::text("TODO: CONFIG")
    ]
    .into()
}
