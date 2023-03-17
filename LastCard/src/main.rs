use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

const SUITS:[&str;4] = ["♥️ ","♦️ ","♠️ ","♣️ "];
const VAULES:[i8;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13];
const FACES:[&str;13] = ["A","2","3","4","5","6","7","8","9","10","J","Q","K"];
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

fn gameloop(){
    
}

fn main(){
    
}

impl DeckType{
    
}

impl Card{
    fn card_out(){

    }
    fn card_compare(){
        
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
    fn play_card(&mut self,deck_to:&mut Deck)->bool{
        if !self.is_empty() && !deck_to.is_empty(){
            deck_to.deck.push(self.deck[0]);
            self.deck.remove(0);
            true
        }
        else{
            false
        }
    }
    fn draw_card(&mut self,deck_from:&mut Deck){
        if !self.is_empty() && !deck_from.is_empty(){
            self.deck.push(deck_from.deck[0]);
            deck_from.deck.remove(0);
        }
    }
}






