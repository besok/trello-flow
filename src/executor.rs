use crate::trello::TrelloManager;
use crate::extractor::{WordTarget, BoardSettingsManager};

struct Executor {
    trello: TrelloManager,
    word_targets: Vec<WordTarget>,
    boards:BoardSettingsManager,
}

impl Executor {

}