use crate::trello::{TrelloManager, Card};
use crate::extractor::{WordTarget, BoardSettingsManager, Extractor};
use std::collections::HashMap;
use crate::matcher::WordMatcher;
use crate::dict::DictManager;

pub struct Executor {}

impl Executor {
    fn compare<'a>(word: &str, cards: &'a Vec<Card>, prob: f32) -> Option<&'a Card> {
        for c in cards {
            if WordMatcher::math_words(word, c.name.as_str(), prob) {
                return Some(c);
            }
        }
        None
    }

    pub fn execute(extractor: Extractor) {
        let manager = extractor.match_board_settings();
        let word_targets = extractor.match_boards();
        let boards_ids: Vec<String> = manager.boards.keys().cloned().collect();
        let words: HashMap<&String, Vec<Card>> = boards_ids.iter().map(|b| (b, extractor.trello.cards(b))).collect();
        let probe = manager.match_f;

        for wt in word_targets.iter() {
            let w = wt.word.as_str();
            if let Some(cards) = words.get(&wt.boards_id) {
                if let Some(card) = Executor::compare(w, cards, probe) {
                    let list_id = manager.boards.get(wt.boards_id.as_str()).map(|s| s.upd_card_list.to_string()).expect("to find upd list");
                    extractor.trello.update_card_list(card.id.as_str(), list_id.as_str());
                    extractor.trello.update_card_dsc(card.id.as_str(), format!("{}\n- {}", card.desc, w).as_str());
                    println!("upd card: {}", w);
                } else {
                    let list_id = manager.boards.get(wt.boards_id.as_str()).map(|s| s.new_card_list.to_string()).expect("to find new list");
                    extractor.trello.create_card(list_id.as_str(), w);
                    println!("new card is created: {}", w);
                }
            }
        }
    }
}

mod tests {
    use crate::extractor::Extractor;
    use crate::executor::Executor;

    #[test]
    fn simple_test() {
        let cred = "/Users/boriszhguchev/projects/trello-vocab-loader/example/trello_token.json";
        let cfg = "/Users/boriszhguchev/projects/trello-vocab-loader/example/cfg.json";
        let data = "/Users/boriszhguchev/projects/trello-vocab-loader/example/1_test.csv";
        Executor::execute(Extractor::new_from(cfg, cred, data))
    }
}
