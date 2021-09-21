use serde::{Serialize, Deserialize};
use std::borrow::{Borrow, BorrowMut};

struct TrelloManager {
    prefix: &'static str,
    key: &'static str,
    token: &'static str,

}

#[derive(Serialize, Deserialize, Debug)]
struct Board {
    id: String,
    name: String,
    closed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct List {
    id: String,
    name: String,
}

impl TrelloManager {
    fn new(key: &'static str, token: &'static str) -> Self {
        TrelloManager { prefix: "https://api.trello.com", key, token }
    }

    fn get_req<'a, T>(&self, url: &str) -> std::io::Result<T>
        where T: for<'de> Deserialize<'de> {
        ureq::get(format!("{}{}", self.prefix, url).as_str())
            .query("key", self.key)
            .query("token", self.token)
            .set("Accept", "application/json")
            .call()
            .expect("should get the result")
            .into_json::<T>()
    }
    fn post_req<'a, T>(&self, url: &str, params: Vec<(&str, &str)>) -> std::io::Result<T>
        where T: for<'de> Deserialize<'de> {
        let mut r = ureq::post(format!("{}{}", self.prefix, url).as_str())
            .query("key", self.key)
            .query("token", self.token)
            .set("Accept", "application/json");

        for (k, v) in params.into_iter() {
            r = r.query(k, v);
        }
        r.call().expect("should get the result").into_json::<T>()
    }


    fn boards(&self) -> Vec<Board> {
        self.get_req::<Vec<Board>>("/1/members/me/boards")
            .expect("get boards")
            .into_iter()
            .filter(|b| !b.closed)
            .collect()
    }
    fn cards(&self, board_id: &str) -> Vec<Card> {
        self.get_req::<Vec<Card>>(format!("/1/boards/{}/cards", board_id).as_str()).expect("get cards")
    }
    fn lists(&self, board_id: &str) -> Vec<List> {
        self.get_req::<Vec<List>>(format!("/1/boards/{}/lists", board_id).as_str()).expect("get lists")
    }
    fn create_card(&self, list_id: &str, card_name: &str) -> Card {
        self.post_req::<Card>(format!("/1//cards").as_str(),
                              vec![("pos", "bottom"), ("idList", list_id), ("name", card_name)])
            .expect("create card")
    }
}


mod tests {
    use crate::trello::TrelloManager;

    #[test]
    fn boards_test() {
        let trello = TrelloManager::new("6ee6c85be50cf98a9d06ff25fdaf6809",
                                        "9d046d8b9565a78846e49c233b2cd14518a46b454995e915eb9c70f5c2d6c835");
        let boards = trello.boards();
        println!("{:?}", boards)
    }

    #[test]
    fn cards_test() {
        let trello = TrelloManager::new("6ee6c85be50cf98a9d06ff25fdaf6809",
                                        "9d046d8b9565a78846e49c233b2cd14518a46b454995e915eb9c70f5c2d6c835");
        let boards = trello.boards();
        println!("{:?}", boards);

        for b in boards.iter() {
            let cards = trello.cards(b.id.as_str());

            println!("{:?}", cards);
        }
    }

    #[test]
    fn lists_test() {
        let trello = TrelloManager::new("6ee6c85be50cf98a9d06ff25fdaf6809",
                                        "9d046d8b9565a78846e49c233b2cd14518a46b454995e915eb9c70f5c2d6c835");
        let boards = trello.boards();
        println!("{:?}", boards);

        for b in boards.iter() {
            let lists = trello.lists(b.id.as_str());

            println!("{:?}", lists);
        }
    }

    #[test]
    fn create_card_test() {
        let trello = TrelloManager::new("6ee6c85be50cf98a9d06ff25fdaf6809",
                                        "9d046d8b9565a78846e49c233b2cd14518a46b454995e915eb9c70f5c2d6c835");
        let dev_id =
            trello.boards().into_iter().find(|b| b.name.eq("DEV")).map(|b| b.id).expect("");

        let l_id = trello.lists(dev_id.as_str()).into_iter().find(|l| l.name.eq("Progress")).map(|l| l.id).expect("");
        trello.create_card(l_id.as_str(),"test2");
    }
}