use std::io; // implentation for user inputs
// allows for shuffling of vectors/the deck
use rand;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;
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
//card struct with main info value,suit, and face
#[derive(Clone,Copy)]
struct Card{
   value:i8,
   suit:&'static str,
   face:&'static str,
}
//deck struct the deck and it's type
#[derive(Clone)]
struct Deck{
   deck:Vec<Card>,
   deckType:DeckType,
}
//gets a string input when needed
fn get_input()->String{
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("invalied input");
   input
}
//turns a String to a i32 is used to convert a string input to i32
fn to_i32 (input:String)->i32{
   let input:i32 = match input.trim().parse(){
       Ok(num)=>num,
       Err(num)=>0,
   };
   return input
}
//the players turn takes all decks in
fn player_turn(discard_deck:&mut Deck,player1_deck:&mut Deck,draw_deck:&mut Deck,player2_deck:&mut Deck){
   // shows the top card of discard pile for the player to play on uesing card_output()
   println!("top of discard pile:");
   discard_deck.deck[discard_deck.deck.len()-1].card_output();
   //checks if any of the cards in the players hands can be played
   if player1_deck.clone().can_play(discard_deck.deck[discard_deck.deck.len()-1]){
       // shows the players cards in a list with the index next to them
       println!("your cards are:");
       for (index,card) in player1_deck.deck.clone().iter().enumerate(){
           print!("{}:",index+1);
           card.card_output();
       }
       println!("play one of your cards");
       // loop to check vaild input
       'outer:loop{
           //gets input
           let card_choice = get_input();
           //turns to a i32
           let card_choice =to_i32(card_choice);
           // shows the player the didn't draw
           if player1_deck.clone().can_play(discard_deck.deck[discard_deck.deck.len()-1]){
               println!("didn't draw");
           }
           //shows the player they did draw
           else{
               println!("drew a card");
               player1_deck.draw_card(draw_deck);
               break 'outer;
           }
           // checks data validity
           if card_choice == 0{
               println!("enter a number between 1 and {}",player1_deck.deck.len());
               continue;
           }
           //runs the compare function to ckeck if the card choosen can be played which returns true or false in the first index of a tuple
           let compare = discard_deck.deck[discard_deck.deck.len()-1].card_compare(player1_deck.deck[(card_choice-1) as usize]);
           //checks the first index of the tuple and if so playes the card
           if compare.0{
               // plays the card that was picked
               player1_deck.play_card(discard_deck,(card_choice-1) as usize);
               // checks if the card was special
               if discard_deck.clone().is_special().0{
                   // checks if the card was an 8
                   if discard_deck.deck[discard_deck.deck.len()-1].value == 8{
                       // runs the player turn which skips the computers turn
                       player_turn(discard_deck, player1_deck, draw_deck, player2_deck);
                   }
                   //check if the card played is a 2
                    else if discard_deck.deck[discard_deck.deck.len()-1].value == 2{
                       // makes the computer draw two cards
                       for i in 0..2{
                           player2_deck.draw_card(draw_deck);
                           println!("player2 drew a card");
                       }
                   }
                   //checks if the card played was an ace
                    else if discard_deck.deck[discard_deck.deck.len()-1].value == 1{
                        println!("ace");
                        discard_deck.deck[discard_deck.deck.len()-1].card_output();
                        println!("{}",discard_deck.deck[discard_deck.deck.len()-1].value);
                       //creates a deck to play from to change the suit
                       let mut holder_deck = Deck{
                           deckType:DeckType::Draw,
                           deck:vec![Card{value:1,face:"A",suit:"♥️ "},Card{value:1,face:"A",suit:"♦️ "},Card{value:1,face:"A",suit:"♠️ "},Card{value:1,face:"A",suit:"♣️ "}],
                       };
                       // input vaility looop
                       loop{
                           println!("enter 1:hearts \n 2:diamonds \n 3:spades \n 4:clubs");
                           //gets and input from the player which should be 1,2,3, or 4
                           let input = get_input();
                           // converts the input to an i32
                           let input:i32 = to_i32(input);
                           if input == 1{
                               //if the input was 1 an A of hearts is played which changes the suit to hearts
                               holder_deck.play_card(discard_deck, (input-1)as usize);
                               //breaks out of input check loop
                               break;
                           }
                           else if input == 2{
                               // if the input was 2 then a A of diamonds is played which changes the suit to diamonds
                               holder_deck.play_card(discard_deck, (input-1)as usize);
                               break;
                           }
                           else if input == 3{
                               // if the input was 3 then a A of spades is played which changes the suit to spades
                               holder_deck.play_card(discard_deck, (input-1)as usize);
                               break;
                           }
                           else if input == 4{
                               //if the input was 4 then a A of clubs is played which changes the suit to spades
                               holder_deck.play_card(discard_deck, (input-1)as usize);
                               break;
                           }
                           else{
                               //if the input was not a 1,2,3, or 4 then ask for a vaild input
                               println!("you have to enter a number between 1-4");
                               continue;
                           }
                       }
                   }
                   // checks if the card played was a joker
                    else if discard_deck.deck[discard_deck.deck.len()-1].value == 0{
                        println!("joker");
                        discard_deck.deck[discard_deck.deck.len()-1].card_output();
                        println!("{}",discard_deck.deck[discard_deck.deck.len()-1].value);
                       // loops six times for the amount of cards to be drawn
                       for i in 0..6{
                           // makes player2 draw the card
                           player2_deck.draw_card(draw_deck);
                           println!("player2 drew a card");
                       }
                       // makes a deck to be played from to change the suit
                       let mut holder_deck = Deck{
                           deckType:DeckType::Draw,
                           deck:vec![Card{value:0,face:"joker",suit:"♥️ "},Card{value:0,face:"joker",suit:"♦️ "},Card{value:0,face:"joker",suit:"♠️ "},Card{value:0,face:"joker",suit:"♣️ "}],
                       };
                       // input vaility looop
                       loop{
                           println!("enter 1:hearts \n 2:diamonds \n 3:spades \n 4:clubs");
                           // gets an input should be 1,2,3,or 4
                           let input = get_input();
                           //converts input to i32
                           let input:i32 = to_i32(input);
                           if input == 1{
                               //if the input was 1 playes a joker with a suit of hearts
                               holder_deck.play_card(discard_deck, (input-1)as usize);
                               break;
                           }
                           else if input == 2{
                               //if the input was 2 playes a joker with a suit of diamonds
                               holder_deck.play_card(discard_deck, (input-1)as usize);
                               break;
                           }
                           else if input == 3{
                               //if the input was 3 playes a joker with a suit of spades
                               holder_deck.play_card(discard_deck, (input-1)as usize);
                               break;
                           }
                           else if input == 4{
                               //if the input was 4 playes a joker with a suit of clubs
                               holder_deck.play_card(discard_deck, (input-1)as usize);
                               break;
                           }
                           else{
                               // if the input was not vaild asks for a vaild output
                               println!("you have to enter a number between 1-4");
                               continue;
                           }


                       }
                   }
                    else if discard_deck.deck[discard_deck.deck.len()-1].value == 11{
                       // if a jack was played lets the player play again
                       player_turn(discard_deck, player1_deck, draw_deck, player2_deck);
                   }
               }
               break;
           }
           else{
               // if the card picked could not be played asks for a vaild card and tells them what is playeble
               println!("you have to play a {}, {},or a wild",discard_deck.deck[discard_deck.deck.len()-1].face,discard_deck.deck[discard_deck.deck.len()-1].suit,);
               continue;
           }
       };
   }
   else{
       // if a card could not be played the player draws a card
       println!("player 1 drew a card");
       player1_deck.draw_card(draw_deck);
   }
}
fn computer_turn(player2_deck:&mut Deck,discard_deck:&mut Deck,draw_deck:&mut Deck,player1_deck:&mut Deck){
   // checks if the computer can play a card
   if player2_deck.clone().can_play(discard_deck.deck[discard_deck.deck.len()-1]){
       //goes over the conputer hand till a card that can be played is reached
       for (index,card) in player2_deck.deck.clone().iter().enumerate(){
           if (discard_deck.deck[discard_deck.deck.len()-1].card_compare(card.clone())).0{
               // plays the card that was playeble
               player2_deck.play_card(discard_deck, index);
               //checks if the card played was an 8
               if discard_deck.deck[discard_deck.deck.len()-1].value == 8{
                   // runs the computer turn which skips the players turn
                   computer_turn(discard_deck, player1_deck, draw_deck, player2_deck);
               }
                //check if the card played is a 2
                else if discard_deck.deck[discard_deck.deck.len()-1].value == 2{
                   // makes the computer draw two cards
                   for i in 0..2{
                       player1_deck.draw_card(draw_deck);
                       println!("player2 drew a card");
                   }
               }
               //checks if the card played was an ace
               else if discard_deck.deck[discard_deck.deck.len()-1].value == 1{
                println!("ace");
                   //creates a deck to play from to change the suit
                   let mut holder_deck = Deck{
                       deckType:DeckType::Draw,
                       deck:vec![Card{value:1,face:"A",suit:"♥️ "},Card{value:1,face:"A",suit:"♦️ "},Card{value:1,face:"A",suit:"♠️ "},Card{value:1,face:"A",suit:"♣️ "}],
                   };
                   // input vaility looop
                   loop{
                       let mut rng = rand::thread_rng();
                       let input = rng.gen_range(1..=4);
                       if input == 1{
                           //if the input was 1 an A of hearts is played which changes the suit to hearts
                           holder_deck.play_card(discard_deck, (input-1)as usize);
                           //breaks out of input check loop
                           break;
                       }
                       else if input == 2{
                           // if the input was 2 then a A of diamonds is played which changes the suit to diamonds
                           holder_deck.play_card(discard_deck, (input-1)as usize);
                           break;
                       }
                       else if input == 3{
                           // if the input was 3 then a A of spades is played which changes the suit to spades
                           holder_deck.play_card(discard_deck, (input-1)as usize);
                           break;
                       }
                       else if input == 4{
                           //if the input was 4 then a A of clubs is played which changes the suit to spades
                           holder_deck.play_card(discard_deck, (input-1)as usize);
                           break;
                       }
                       else{
                           //if the input was not a 1,2,3, or 4 then ask for a vaild input
                           println!("you have to enter a number between 1-4");
                           continue;
                       }
                   }
               }
               // checks if the card played was a joker
               else if discard_deck.deck[discard_deck.deck.len()-1].value == 0{
                    println!("joker");
                   // loops six times for the amount of cards to be drawn
                   for i in 0..6{
                       // makes player2 draw the card
                       player1_deck.draw_card(draw_deck);
                       println!("player2 drew a card");
                   }
                   // makes a deck to be played from to change the suit
                   let mut holder_deck = Deck{
                       deckType:DeckType::Draw,
                       deck:vec![Card{value:0,face:"joker",suit:"♥️ "},Card{value:0,face:"joker",suit:"♦️ "},Card{value:0,face:"joker",suit:"♠️ "},Card{value:0,face:"joker",suit:"♣️ "}],
                   };
                   // input vaility looop
                   loop{
                        let mut rng = rand::thread_rng();
                        let input = rng.gen_range(1..=4);
                       if input == 1{
                           //if the input was 1 playes a joker with a suit of hearts
                           holder_deck.play_card(discard_deck, (input-1)as usize);
                           break;
                       }
                       else if input == 2{
                           //if the input was 2 playes a joker with a suit of diamonds
                           holder_deck.play_card(discard_deck, (input-1)as usize);
                           break;
                       }
                       else if input == 3{
                           //if the input was 3 playes a joker with a suit of spades
                           holder_deck.play_card(discard_deck, (input-1)as usize);
                           break;
                       }
                       else if input == 4{
                           //if the input was 4 playes a joker with a suit of clubs
                           holder_deck.play_card(discard_deck, (input-1)as usize);
                           break;
                       }
                       else{
                           // if the input was not vaild asks for a vaild output
                           println!("you have to enter a number between 1-4");
                           continue;
                       }


                   }
               }
               //checks if the card played was a jack
               else if discard_deck.deck[discard_deck.deck.len()-1].value == 11{
                   // lets the computer play again
                   computer_turn(discard_deck, player1_deck, draw_deck, player2_deck);
               }
               break;
           }
           else{
           }
       }


   }
   else{
       // if the computer has not playeble cards makes it draw a card
       println!("computer drew card");
       player2_deck.draw_card(draw_deck);
   }
}


fn gameloop(){
   // creates the draw deck with a blank vector
   let mut draw_deck = Deck{
       deck:Vec::new(),
       deckType:DeckType::Draw,
   };
   // creates the deck with to standared 54 card decks
   draw_deck.innit_54_deck();
   draw_deck.innit_54_deck();
   // creates a discard deck which is empty
   let mut discard_deck = Deck{
       deck:Vec::new(),
       deckType:DeckType::Discard,
   };
   //creates the player deck which is empty
   let mut player1_deck = Deck{
       deck:Vec::new(),
       deckType:DeckType::Player1,
   };
   //creates the second player or computer deack which is empty
   let mut player2_deck = Deck{
       deck:Vec::new(),
       deckType:DeckType::Player2,
   };
   // shuffles the deck
   draw_deck.shuffle();
   // draws one card of the draw deck and adds it to the discard deck
   discard_deck.draw_card(&mut draw_deck);
   //gives each players each 5 cards
   for index in 0..=5{
       player1_deck.draw_card(&mut draw_deck);
       player2_deck.draw_card(&mut draw_deck);
       draw_deck.shuffle();
   }
   // the gameloop
   loop{
       // runs the player turn
       player_turn(&mut discard_deck, &mut player1_deck,&mut draw_deck,&mut player2_deck);
       // runs the computers turn
       computer_turn(&mut player2_deck, &mut discard_deck, &mut draw_deck,&mut player1_deck);
       if draw_deck.deck.len() <= 1{
           for (index,card) in discard_deck.deck.clone().iter().enumerate(){
               draw_deck.deck.push(discard_deck.deck[index]);
               discard_deck.deck.remove(index);
           }
       }
       //checks the win conditions by checking the length of the players hand then breaks out of the loop
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
    // allows the ansi escape codes to work for windows by activating it 
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {
            // ANSI escape codes were successfully enabled, or this is a non-Windows platform.
            // Use your terminal color library of choice here.
        }
        Err(_) => {
            // The operation was unsuccessful, typically because it's running on an older
            // version of Windows. The program may choose to disable ANSI color code output in
            // this case.
        }
    }
   loop{
       gameloop();
       break;
   }
}


impl DeckType{
  
}


impl Card{
   // outputs the card with style of ansii escape codes
   fn card_output(&self){
       // the red code for terminals
       let red = "\x1b[1;31m";
       // the black code for terminals
       let black ="\x1b[1;1m";
       // the reset code for terminals
       let reset = "\x1b[0m";
       //prints the card red if the suit is haerts or diamonds
       if self.suit == "♥️ " || self.suit == "♦️ "{
           println!("{}{}{}{}",red,self.face,self.suit,reset);
       }
       // prints the card black if the suit is spades or clubs
       else if self.suit == "♠️ " || self.suit == "♣️ "{
           println!("{}{}{}{}",black,self.face,self.suit,reset)
       }
   }
   //compares two cards and returns a bool and a string
   fn card_compare(&self,other_card:Card)->(bool,&str){
       // if the value of both cards match then returns true and value
       if self.value == other_card.value{
           return (true,"value");
       }
       // if the suit of both cards match then returns true
       if self.suit == other_card.suit{
           return (true,"suit");
       }
       // if the other card or the played card is wild in other words ace or joker return true and wild
       if other_card.value == 0 || other_card.value == 1{
           return (true,"wild");
       }
       // if none of the above statments are reached false is returned
       (false,"false")
   }
}


impl Deck{
   //creates a 52 card deck
   fn innit_52_deck(&mut self){
       // loops through the possible suits
       for suit in SUITS.iter(){
           //loops through the possible card faces eg A,2,3,J,Q,K
           for (index,face) in FACES.iter().enumerate(){
               //creates a card with the suit face and value needed
               let mut card = Card{
                   suit:suit,
                   face:face,
                   value:VAULES[index]
               };
               // addeds the card to the deck self
               self.deck.push(card);
           }
       }
   }
   //creates a standard 54 card deck
   fn innit_54_deck(&mut self){
       //creates the 52 card deck
       self.innit_52_deck();
       //adds a black joker
       let mut joker_black = Card{
           suit:"♠️ ",
           value:0,
           face:"joker"
       };
       //adds a red joker
       let mut joker_red = Card{
           suit:"♥️ ",
           value:0,
           face:"joker",
       };
       // adds both joker to the deck of this instance
       self.deck.push(joker_black);
       self.deck.push(joker_red);
   }
   //checks if the deck is empty
   fn is_empty(&mut self)->bool{
       if self.deck.len() ==0{
           true
       }
       else{
           false
       }
   }
   //shuffles the deck using thread_rng()
   fn shuffle(&mut self){
       self.deck.shuffle(&mut thread_rng());
   }
   //plays a card from the current deck to another deck from a index of this deck
   fn play_card(&mut self,deck_to:&mut Deck,index:usize)->bool{
       // check that the deck is not empty
       if !self.is_empty(){
           //plays the card by adding it to the other deck
           deck_to.deck.push(self.deck[index]);
           // then removes it from this deck
           self.deck.remove(index);
           true
       }
       else{
           false
       }
   }
   //draws a card from the top of another deck
   fn draw_card(&mut self,deck_from:&mut Deck){
       //checks the decks is not empty
       if !deck_from.is_empty(){
           // draws the card to this deck
           self.deck.push(deck_from.deck[0]);
           //removes it from the other deck
           deck_from.deck.remove(0);
       }
   }
   // checks if a card can be played from a hand on another card
   fn can_play(self,card:Card)-> bool{
       //loops through the deck
       for (index,card_loop) in self.deck.iter().enumerate(){
           // returns true if any of the cards can be played
           if (card_loop.card_compare(card)).0{
               return true;
           }
       }
       //return false if none can be played
       return false;
   }
   // checks if the card is special
   fn is_special(self)-> (bool,&'static str){
       //runs through every card
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
}
