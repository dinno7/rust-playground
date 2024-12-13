use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards: Vec<String> = vec![];

        for suit in suits {
            for value in values {
                cards.push(format!("{value} of {suit}"));
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("The deck {:#?}", deck);
}
