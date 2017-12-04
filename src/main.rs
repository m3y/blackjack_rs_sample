extern crate rand;

use std::fmt;
use rand::Rng;

#[derive(Debug)]
enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Debug)]
struct Rank(i32);

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            1 => write!(f, "{}", "A"),
            11 => write!(f, "{}", "J"),
            12 => write!(f, "{}", "Q"),
            13 => write!(f, "{}", "K"),
            _ => write!(f, "{}", self.0),
        }
    }
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit: suit,
            rank: rank,
        }
    }

    fn show(&self) -> String {
        match self.suit {
            Suit::Spade => format!("{}{}", "S", self.rank),
            Suit::Heart => format!("{}{}", "H", self.rank),
            Suit::Club => format!("{}{}", "C", self.rank),
            Suit::Diamond => format!("{}{}", "D", self.rank),
        }
    }
}

#[derive(Debug)]
struct Deck {
    // ToDo: 引かれたカードを記録しておきたい
}

impl Deck {
    fn new() -> Deck {
        Deck {
        }
    }

    fn draw(&self) -> Card {
        // ToDo: 引かれていないカードを返却するようにしたい
        Card::new(self.chose_suit(), self.chose_rank())
    }

    fn chose_suit(&self) -> Suit {
        let suit_number = rand::thread_rng().gen_range(1,4);
        match suit_number {
            1 => Suit::Spade,
            2 => Suit::Heart,
            3 => Suit::Club,
            4 => Suit::Diamond,
            _ => Suit::Spade,
        }
    }

    fn chose_rank(&self) -> Rank {
        let rank_value = rand::thread_rng().gen_range(1,13);
        Rank(rank_value)
    }

    // ToDo: 引かれていないカード、引かれたカードを一覧したい
}

// 手札
// struct Hand {}

// 捨て札
// struct Discard {}

// ゲーム(??)
// struct Game {}

// 役の判定？

fn main() {
    let deck = Deck::new();
    let card = deck.draw();
    println!("{:?}", card.show());
    let card_s = Card::new(Suit::Spade, Rank(1));
    let card_h = Card::new(Suit::Heart, Rank(13));
    let card_c = Card::new(Suit::Club, Rank(12));
    let card_d = Card::new(Suit::Diamond, Rank(8));
    println!("{:?}", card_s.show());
    println!("{:?}", card_h.show());
    println!("{:?}", card_c.show());
    println!("{:?}", card_d.show());
}
