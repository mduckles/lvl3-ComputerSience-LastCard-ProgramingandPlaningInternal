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


