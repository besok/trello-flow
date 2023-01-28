mod settings;
mod state;

use iced::{Application, Command, Theme};

use crate::trello::TrelloConnector;

use self::{settings::Settings, state::State};

pub struct Executor {
    settings: Settings,
    connector: TrelloConnector,
    state: State,
}

impl Default for Executor {
    fn default() -> Self {
        let settings = Settings::default();
        let connector = TrelloConnector::from_file(settings.credentials_path.as_str());
        let state = State::default();
        Self {
            settings,
            connector,
            state,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Init(State),
}

impl Application for Executor {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Executor::default(),
            Command::perform(state::init_state(), Message::Init),
        )
    }

    fn title(&self) -> String {
        format!("Board:{}", self.state.board_name)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        todo!()
    }
}
