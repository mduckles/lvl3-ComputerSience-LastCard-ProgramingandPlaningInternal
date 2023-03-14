use std::io;

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

impl DeckType{
    
}

impl Card{
    fn card_out(){
        
    }
    fn card_compare(){
        
    }
}

impl Deck{
    fn innit_52_deck(){
        
    }
    fn innit_54_deck(){
        
    }
    fn shuffle(){
        
    }
    fn play_card(){

    }
    fn draw_card(){

    }
    fn is_empty(){
        
    }
}

fn gameloop(){
    
}

fn main(){
    
}


