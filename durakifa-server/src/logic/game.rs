use std::collections::HashMap;

use durakifa_protocol::{Card, Deck, Hand, Rank};
use naia_bevy_server::UserKey;

pub struct Game {
    pub deck: Deck,
    pub hands: HashMap<UserKey, Hand>,
}

impl Game {
    pub fn new(players: Vec<UserKey>) -> Game {
        let cards = Card::all_cards()
            .into_iter()
            .filter(|c| c.rank >= Rank::Six)
            .map(|c| *c)
            .collect::<Vec<Card>>();

        let mut hands = HashMap::new();
        for k in players {
            hands.insert(k, Hand::new());
        }

        Game {
            deck: Deck::from_cards(&cards),
            hands,
        }
    }
}
