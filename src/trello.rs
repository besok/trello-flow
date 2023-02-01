use crate::files::{read_file_into_string, yml_str_to};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, BorrowMut};

pub struct TrelloConnector {
    prefix: &'static str,
    cred: TrelloCred,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrelloCred {
    key: String,
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub id_list: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub id: String,
    pub name: String,
}

impl TrelloConnector {
    pub fn from_file(path: &str) -> Self {
        let cred = read_file_into_string(path)
            .map(|s| yml_str_to(&s, "cred should have token and key"))
            .expect("the cfg file should exist");

        TrelloConnector {
            prefix: "https://api.trello.com",
            cred,
        }
    }
}

impl TrelloConnector {
    fn get_req<T>(&self, url: &str) -> std::io::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        ureq::get(format!("{}{}", self.prefix, url).as_str())
            .query("key", self.cred.key.as_str())
            .query("token", self.cred.token.as_str())
            .set("Accept", "application/json")
            .call()
            .expect("should get the result")
            .into_json::<T>()
    }
    fn post_req<T>(&self, url: &str, params: Vec<(&str, &str)>) -> std::io::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut r = ureq::post(format!("{}{}", self.prefix, url).as_str())
            .query("key", self.cred.key.as_str())
            .query("token", self.cred.token.as_str())
            .set("Accept", "application/json");

        for (k, v) in params.into_iter() {
            r = r.query(k, v);
        }
        r.call().expect("should get the result").into_json::<T>()
    }

    fn put_req<T>(&self, url: &str, params: Vec<(&str, &str)>) -> std::io::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut r = ureq::put(format!("{}{}", self.prefix, url).as_str())
            .query("key", self.cred.key.as_str())
            .query("token", self.cred.token.as_str())
            .set("Accept", "application/json");

        for (k, v) in params.into_iter() {
            r = r.query(k, v);
        }
        r.call().expect("should get the result").into_json::<T>()
    }
}

impl TrelloConnector {
    pub fn boards(&self) -> Vec<Board> {
        self.get_req::<Vec<Board>>("/1/members/me/boards")
            .expect("get boards")
            .into_iter()
            .filter(|b| !b.closed)
            .collect()
    }
    pub fn cards(&self, board_id: &str) -> Vec<Card> {
        self.get_req::<Vec<Card>>(format!("/1/boards/{}/cards", board_id).as_str())
            .expect("get cards")
    }
    pub fn lists(&self, board_id: &str) -> Vec<List> {
        self.get_req::<Vec<List>>(format!("/1/boards/{}/lists", board_id).as_str())
            .expect("get lists")
    }

    pub fn list_by_name(&self, board_id: &str, name: &str) -> Option<List> {
        self.get_req::<Vec<List>>(format!("/1/boards/{}/lists", board_id).as_str())
            .expect("get lists")
            .into_iter()
            .find(|l| l.name == name)
    }

    pub fn create_card(&self, list_id: &str, card_name: &str) -> Card {
        self.post_req::<Card>(
            format!("/1/cards").as_str(),
            vec![("pos", "bottom"), ("idList", list_id), ("name", card_name)],
        )
        .expect("create card")
    }
    pub fn update_card_name(&self, id: &str, new_name: &str) -> Card {
        self.put_req::<Card>(
            format!("/1/cards/{}", id).as_str(),
            vec![("name", new_name)],
        )
        .expect("update card")
    }
    pub fn update_card_list(&self, card_id: &str, card_list: &str) -> Card {
        self.put_req::<Card>(
            format!("/1/cards/{}", card_id).as_str(),
            vec![("idList", card_list)],
        )
        .expect("update card list")
    }
    pub fn update_card_dsc(&self, card_id: &str, desc: &str) -> Card {
        self.put_req::<Card>(
            format!("/1/cards/{}", card_id).as_str(),
            vec![("desc", desc)],
        )
        .expect("update card desc")
    }
}

mod tests {
    use crate::trello::{TrelloConnector, TrelloCred};

    #[test]
    fn boards_test() {
        let trello = TrelloConnector::from_file(
            "/home/bzhg/projects/trello-vocab-loader/examples/trello_cred.yml",
        );
        let boards = trello.boards();
        println!("{:?}", boards)
    }

    #[test]
    fn cards_test() {
        let trello = TrelloConnector::from_file(
            "/home/bzhg/projects/trello-vocab-loader/examples/trello_cred.yml",
        );
        let boards = trello.boards();
        println!("{:?}", boards);

        for b in boards.iter() {
            let cards = trello.cards(b.id.as_str());

            println!("{:?}", cards);
        }
    }

    #[test]
    fn lists_test() {
        let trello = TrelloConnector::from_file(
            "/Users/boriszhguchev/projects/trello-vocab-loader/example/trello_token.json",
        );
        let boards = trello.boards();
        println!("{:?}", boards);

        for b in boards.iter() {
            let lists = trello.lists(b.id.as_str());

            println!("{:?}", lists);
        }
    }

    #[test]
    fn update_card_test() {
        let trello = TrelloConnector::from_file("~/.trello/cred.json");
        let eng_id = trello
            .boards()
            .into_iter()
            .find(|b| b.name.eq("ENG"))
            .map(|b| b.id)
            .expect("");
        let cards = trello.cards(eng_id.as_str());
        let c = cards.first().unwrap();
        let lists = trello.lists(&*eng_id);
        let list = lists.iter().find(|l| l.name == "Queue").unwrap();

        trello.update_card_list(c.id.as_str(), list.id.as_str());
        println!("card:{}", c.name)
    }
}
