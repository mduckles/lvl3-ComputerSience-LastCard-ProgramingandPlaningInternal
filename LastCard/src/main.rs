use core::num;
use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;
// values for making 52 and 54 card deck
const SUITS:[&str;4] = ["♥️ ","♦️ ","♠️ ","♣️ "];
const VAULES:[i8;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13];
const FACES:[&str;13] = ["A","2","3","4","5","6","7","8","9","10","J","Q","K"];
//to dishern deck types
#[derive(Clone)]
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
#[derive(Clone)]
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
fn player_turn(discard_deck:&mut Deck,player1_deck:&mut Deck,draw_deck:&mut Deck){
    if discard_deck.clone().is_special().0{
        if discard_deck.deck[discard_deck.deck.len()-1].value == 0{
            for i in 0..6{
                player1_deck.draw_card(draw_deck);
            }
        }
    }
    discard_deck.deck[discard_deck.deck.len()-1].card_output();
    if player1_deck.clone().can_play(discard_deck.deck[discard_deck.deck.len()-1]){
            println!("your cards are:");
        for (index,card) in player1_deck.deck.clone().iter().enumerate(){
            print!("{}:",index+1);
            card.card_output();
        }
        println!("play one of your cards");
        'outer:loop{
            let card_choice = get_input();
            let card_choice =to_i32(card_choice);
            let mut from_hand=false;
            if player1_deck.clone().can_play(discard_deck.deck[discard_deck.deck.len()-1]){
                println!("didn't draw");
            }
            else{
                println!("drew a card");
                player1_deck.draw_card(draw_deck);
                break 'outer;
            }
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
                player1_deck.play_card(discard_deck,(card_choice-1) as usize);
                break;
            }
            else{
                println!("you have to play a {}, {},or a wild",discard_deck.deck[discard_deck.deck.len()-1].face,discard_deck.deck[discard_deck.deck.len()-1].suit,);
                continue;
            }
        };
    }
    else{
        println!("player 1 drew a card");
        player1_deck.draw_card(draw_deck);
    }
}
fn computer_turn(player2_deck:&mut Deck,discard_deck:&mut Deck,draw_deck:&mut Deck){
    if player2_deck.clone().can_play(discard_deck.deck[discard_deck.deck.len()-1]){
        for (index,card) in player2_deck.deck.clone().iter().enumerate(){
            if (card.card_compare(discard_deck.deck[discard_deck.deck.len()-1])).0{
                player2_deck.play_card(discard_deck, index);
                break;
            }
            else{
            }
        }

    }
    else{
        println!("computer drew card");
        player2_deck.draw_card(draw_deck);
    }
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
        player_turn(&mut discard_deck, &mut player1_deck,&mut draw_deck);
        computer_turn(&mut player2_deck, &mut discard_deck, &mut draw_deck);
        if player1_deck.deck.len() == 0{
            println!("player1 won");
            break;
        }
        if player2_deck.deck.len() == 0{
            println!("player2 won");
            break;
        }

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
    fn is_special(self)-> (bool,&'static str){
        match self.deck[self.deck.len()-1].value{
            0 => (true,"joker"),
            1 => (true,"ace"),
            2 => (true,"2"),
            3 => (false,"3"),
            4 => (false,"4"),
            5 => (false,"5"),
            6 => (false,"6"),
            7 => (false,"7"),
            8 => (true,"8"),
            9 => (false,"9"),
            10 => (false,"10"),
            11 => (true,"jack"),
            12 => (false,"queen"),
            13 => (false,"King"),
            _ => (false,"invaild")
        }
    }
    fn joker(){

    }
    fn ace(){

    }
    fn two(){

    }
    fn eight(){

    }
    fn jack(){

    }
}