#[derive(Debug, Clone)]
pub struct State {
    pub board_name: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            board_name: "Undefined".to_string(),
        }
    }
}

pub async fn init_state() -> State {
    State::default()
}
