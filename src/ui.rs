use iced::{widget, Element, Error};

const APPLICATION_TITLE: &str = "Fretboard";

pub fn run() -> Result<(), Error> {
    let app = iced::application(APPLICATION_TITLE, update, view);
    app.run()
}

#[derive(Debug)]
enum State {
    A,
}

impl Default for State {
    fn default() -> Self {
        Self::A
    }
}

#[derive(Debug)]
enum Message {}

fn update(state: &mut State, message: Message) {
    println!("STATE: {:?}; MESSAGE: {:?}", state, message)
}

fn view(state: &State) -> Element<Message> {
    widget::text("test").into()
}
