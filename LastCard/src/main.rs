use core::num;
use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;
// values for making 52 and 54 card deck
const SUITS:[&str;4] = ["♥️ ","♦️ ","♠️ ","♣️ "];
const VAULES:[i8;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13];
const FACES:[&str;13] = ["A","2","3","4","5","6","7","8","9","10","J","Q","K"];
//to dishern deck types
enum DeckType{
    Player1,
    Player2,
    Discard,
    Draw,
}

#[derive(Clone,Copy)]
struct Card{
    value:i8,
    suit:&'static str,
    face:&'static str,
}

struct Deck{
    deck:Vec<Card>,
    deckType:DeckType,
}
fn get_input()->String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("invalied input");
    input
}
fn to_i32 (input:String)->i32{
    let input:i32 = match input.trim().parse(){
        Ok(num)=>num,
        Err(num)=>0,
    };
    return input
}

fn gameloop(){
    let mut draw_deck = Deck{
        deck:Vec::new(),
        deckType:DeckType::Draw,
    };
    draw_deck.innit_54_deck();
    draw_deck.innit_54_deck();
    let mut discard_deck = Deck{
        deck:Vec::new(),
        deckType:DeckType::Discard,
    };
    let mut player1_deck = Deck{
        deck:Vec::new(),
        deckType:DeckType::Player1,
    };
    let mut player2_deck = Deck{
        deck:Vec::new(),
        deckType:DeckType::Player2,
    };
    draw_deck.shuffle();
    discard_deck.draw_card(&mut draw_deck);
    for index in 0..=5{
        player1_deck.draw_card(&mut draw_deck);
        player2_deck.draw_card(&mut draw_deck);
        draw_deck.shuffle();
    }
    loop{
        discard_deck.deck[discard_deck.deck.len()-1].card_output();
        println!("your cards are:");
        for (index,card) in player1_deck.deck.iter().enumerate(){
            print!("{}:",index+1);
            card.card_output();
        }
        println!("play one of your cards");
        'outer:loop{
            let card_choice = get_input();
            let card_choice =to_i32(card_choice);
            let mut from_hand = false;
            for (index,card) in player1_deck.deck.iter().enumerate(){
                if card_choice == (index+1) as i32{
                    from_hand = true
                } 
            }
            if card_choice == 0{
                println!("enter a number between 1 and {}",player1_deck.deck.len());
                continue;
            }
            else if !from_hand {
                println!("enter a number between 1 and {}",player1_deck.deck.len());
                continue;
            }
            let compare = discard_deck.deck[discard_deck.deck.len()-1].card_compare(player1_deck.deck[(card_choice-1) as usize]);
            if compare.0{
                player1_deck.play_card(&mut discard_deck,(card_choice-1) as usize);

                break;
            }
            else{
                println!("you have to play a {}, {},or a wild",discard_deck.deck[discard_deck.deck.len()-1].face,discard_deck.deck[discard_deck.deck.len()-1].suit,);
                continue;
            }
        };

    }
}

fn main(){
    loop{
        gameloop();
        break;
    }
}

impl DeckType{
    
}

impl Card{
    fn card_output(&self){
        let red = "\x1b[1;31m";
        let black ="\x1b[1;1m";
        let reset = "\x1b[0m";
        if self.suit == "♥️ " || self.suit == "♦️ "||self.suit =="♥️ ♦️"{
            println!("{}{}{}{}",red,self.face,self.suit,reset);
        }
        else if self.suit == "♠️ " || self.suit == "♣️ " || self.suit =="♠️ ♣️ " {
            println!("{}{}{}{}",black,self.face,self.suit,reset)
        }
    }
    fn card_compare(&self,other_card:Card)->(bool,&str){
        if self.value == other_card.value{
            return (true,"value");
        }
        if self.suit == other_card.suit{
            return (true,"suit");
        }
        if self.value == 0 || other_card.value == 0{
            return (true,"joker");
        }
        (false,"false")
    }
}

impl Deck{
    fn innit_52_deck(&mut self){
        for suit in SUITS.iter(){
            for (index,face) in FACES.iter().enumerate(){
                let mut card = Card{
                    suit:suit,
                    face:face,
                    value:VAULES[index]
                };
                self.deck.push(card);
            }
        }
    }
    fn innit_54_deck(&mut self){
        self.innit_52_deck();
        let mut joker_black = Card{
            suit:"♠️ ♣️ ",
            value:0,
            face:"joker"
        };
        let mut joker_red = Card{
            suit:"♥️ ♦️",
            value:0,
            face:"joker",
        };
        self.deck.push(joker_black);
        self.deck.push(joker_red);
    }
    fn is_empty(&mut self)->bool{
        if self.deck.len() ==0{
            true
        }
        else{
            false
        }
    }
    fn shuffle(&mut self){
        self.deck.shuffle(&mut thread_rng());
    }
    fn play_card(&mut self,deck_to:&mut Deck,index:usize)->bool{
        if !self.is_empty(){
            deck_to.deck.push(self.deck[index]);
            self.deck.remove(index);
            true
        }
        else{
            false
        }
    }
    fn draw_card(&mut self,deck_from:&mut Deck){
        if !deck_from.is_empty(){
            self.deck.push(deck_from.deck[0]);
            deck_from.deck.remove(0);
        }
    }
    fn can_play(self,card:Card)-> bool{
        for (index,card_loop) in self.deck.iter().enumerate(){
            if (card_loop.card_compare(card)).0{
                return true;
            }
        }
        return false;
    }
}