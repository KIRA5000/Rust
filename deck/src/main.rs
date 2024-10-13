use rand::{seq::SliceRandom, thread_rng}; // This is equivalent to writiing 2 lines, i.e: use rand::thread_rng; use rand::seq::SliceRandom

#[derive(Debug)] // It's an attribute line for struct Deck, telling compiller to add additional behaviour to struct, derive is one of the attribute which tells compiler to implement a trait for this struct, Debug is a trait, which is required by Deck struct to use {:?}, as it is written above Deck, it's only for deck structure
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = vec!["Hearts", "Spades", "Diamonds", "Clubs"]; // You can make this array too.
        let numbers = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];

        let mut cards: Vec<String> = Vec::new();

        for suit in suits {
            for number in numbers {
                let card = format!("{} of {}", number, suit);
                cards.push(card);
            }
        }

        // let deck = Deck { cards: cards };
        let deck = Deck { cards }; // This is reffered as shorthand struct initialization, as right hand name is same as left hand name, we can shorten it, works same as above line
                                   // let deck1 = Deck { cards: Vec::new() }; // To declare empty
                                   // You can also write above line as: let deck1 = Deck { cards: vec![] };
        deck

        // return Deck { cards }  // You can write above 2 lines as this too.
    }

    fn suffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num: usize) -> Vec<String> {
        if num < self.cards.len() {
            self.cards.split_off(self.cards.len() - num)
        }
        else {
            Vec::new()
        }
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.suffle();

    println!("Your deck is as follows: {:?}", deck); // {:?} is string formatter (read: https://doc.rust-lang.org/rust-by-example/hello/print.html for more info), telling compiller to print it for debug purposes in debug format.
                                                     // println!("Your deck is as follows: {:#?}", deck1); // To print above in pretty format

    println!("These are the cards you drew:");
    //Need to add error handling
    let ret: Vec<String> = deck.deal(2);
    println!("Your delt cards are: {:#?}", ret);
}
