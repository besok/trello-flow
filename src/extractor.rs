use crate::trello::{TrelloManager, Card};
use crate::dict::DictManager;
use std::iter::Map;
use std::collections::HashMap;

pub struct Extractor {
    pub trello: TrelloManager,
    pub dict: DictManager,
}


pub struct WordTarget {
    pub boards_id: String,
    pub word: String,
}

pub struct BoardSettingsManager {
    pub match_f: f32,
    pub boards: HashMap<String, BoardSettings>,
}

#[derive(Debug)]
pub struct BoardSettings {
    pub new_card_list: String,
    pub upd_card_list: String,
}

impl Extractor {
    pub fn match_board_settings(&self) -> BoardSettingsManager {
        let boards: HashMap<String, BoardSettings> =
            self.trello
                .boards()
                .iter()
                .map(|b| {
                    let lists = self.trello.lists(&b.id);
                    (b.id.to_string(),
                     self.dict.cfg.dicts
                         .iter()
                         .find(|d| d.board == b.name)
                         .map(|d| BoardSettings {
                             new_card_list: lists.iter().find(|l| l.name == d.list_create).map(|l| l.id.to_string()).expect("list for create should be"),
                             upd_card_list: lists.iter().find(|l| l.name == d.list_move).map(|l| l.id.to_string()).expect("list for update should be"),
                         }))
                })
                .filter(|(id, mb)| mb.is_some())
                .map(|(id, mb)| (id, mb.unwrap()))
                .collect();

        BoardSettingsManager {
            match_f: self.dict.cfg.match_f,
            boards,
        }
    }
    pub fn match_boards(&self) -> Vec<WordTarget> {
        let bond_col_board: HashMap<&str, &str> = self.col_by_boards();
        let words: HashMap<&str, &str> = self.words_by_col();
        let boards = self.trello.boards();
        let boards: HashMap<&str, &str> = boards.iter().map(|b| (b.name.as_str(), b.id.as_str())).collect();
        let mut wtargets = vec![];

        for (w, col) in words {
            if let Some(b_id) = bond_col_board.get(col).and_then(|b| boards.get(*b)) {
                wtargets.push(WordTarget {
                    boards_id: String::from(*b_id),
                    word: String::from(w),
                })
            }
        }
        wtargets
    }
    fn col_by_boards(&self) -> HashMap<&str, &str> {
        self.dict.cfg.dicts.iter().map(|d| (d.name.as_str(), d.board.as_str())).collect()
    }
    fn words_by_col(&self) -> HashMap<&str, &str> {
        self.dict.data
            .iter()
            .flat_map(|d| vec![(d.src.as_str(), d.from.as_str()), (d.to.as_str(), d.dst.as_str())])
            .collect()
    }
}

impl Extractor {
    pub fn new_from(cfg: &str, cred: &str, data: &str) -> Self {
        let trello = TrelloManager::from_file(cred);
        let dict = DictManager::new(cfg, data);
        Extractor { trello, dict }
    }
    pub fn new(trello: TrelloManager, dict: DictManager) -> Self {
        Extractor { trello, dict }
    }
    fn clean_up_cards(&self, board_name: &str, start_symbol: char, end_symbol: char) -> Vec<Card> {
        let b_id =
            self.trello.boards().into_iter().find(|b| b.name == board_name)
                .map(|b| b.id)
                .expect(format!("board:{} not found", board_name).as_str());
        let cards = self.trello.cards(&b_id);
        let mut upd_cards = vec![];
        for c in cards.into_iter() {
            let name = c.name;
            if name.contains(start_symbol) || name.contains(end_symbol) {
                let start = name.chars().position(|c| c == start_symbol).expect(format!("{}", start_symbol).as_str());
                let end = name.chars().position(|c| c == end_symbol).expect(format!("{}", end_symbol).as_str());
                let new_name: String = name.chars().take(start).chain(name.chars().skip(end + 1)).collect();
                let new_name = new_name.trim();
                let upd_card = self.trello.update_card_name(c.id.as_str(), new_name);
                println!("find a card with a wrong name:{} -> {}", name, upd_card.name);
                upd_cards.push(upd_card)
            }
        }
        upd_cards
    }
}


mod tests {
    use crate::trello::TrelloManager;
    use crate::extractor::Extractor;
    use crate::dict::DictManager;


    #[test]
    fn match_boards_test() {
        let cred = "/Users/boriszhguchev/projects/trello-vocab-loader/example/trello_token.json";
        let cfg = "/Users/boriszhguchev/projects/trello-vocab-loader/example/cfg.json";
        let data = "/Users/boriszhguchev/projects/trello-vocab-loader/example/data.csv";


        let r = Extractor::new(TrelloManager::from_file(cred), DictManager::new(cfg, data));
        let vec = r.match_boards();
        for wt in vec {
            println!("{} | {}", wt.word, wt.boards_id)
        }
    }

    #[test]
    fn match_boards_settings_test() {
        let cred = "/Users/boriszhguchev/projects/trello-vocab-loader/example/trello_token.json";
        let cfg = "/Users/boriszhguchev/projects/trello-vocab-loader/example/cfg.json";
        let data = "/Users/boriszhguchev/projects/trello-vocab-loader/example/data.csv";


        let r = Extractor::new(TrelloManager::from_file(cred), DictManager::new(cfg, data));
        let man = r.match_board_settings();
        for (id, s) in man.boards {
            println!("{},{:?}", id, s)
        }
    }
}