use std::time::Instant;
use rand::{thread_rng, prelude::SliceRandom};

use crate::board::*;

mod board;

const DEBUG: bool = false;

struct Stats {
    turns: u32,
    winner: u32,
    infinite: bool,
}

struct Summary {
    turns: Vec<u32>, // Number of turns for each game
    winners: Vec<u32>, // A player-sized vector counting how many wins each player has
}

#[derive(Debug)]
pub enum PlayerCount {
    Two,
    Three,
    Four,
}

impl PlayerCount {
    fn value(&self) -> u32 {
        match *self {
            PlayerCount::Two => 2,
            PlayerCount::Three => 3,
            PlayerCount::Four => 4,
        }
    }
}

struct Player {
    order: u32,
    space: usize,
    stuck: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tile {
    Start,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    IceCreamCone,
    Gumdrop,
    Lollipop,
    Peppermint,
    BonBon,
    End,
}

#[derive(Clone, Debug)]
pub struct Card {
    tile: Tile,
    num: u32,
}

/// Calculate the stats from a given number of players/desired games
/// If a deck is provided it will not be shuffled
/// Otherwise, a randomly generated deck will be used
pub fn calculate(p: PlayerCount, num_games: u32, deck: Option<Vec<Card>>) {
    // Setup summary for runs
    let mut s = match p {
        PlayerCount::Two => Summary{turns: Vec::new(), winners: vec![0; 2]},
        PlayerCount::Three => Summary{turns: Vec::new(), winners: vec![0; 3]},
        PlayerCount::Four => Summary{turns: Vec::new(), winners: vec![0; 4]},
    };
    
    // Get current time and run the desired number of games
    let now = Instant::now();
    for _ in 0..num_games {
        let stats = play(&p, deck.clone());
        if stats.infinite {
            return;
        }
        s.turns.push(stats.turns);
        *s.winners.get_mut((stats.winner - 1) as usize).unwrap() += 1;
    }
    let elapsed = now.elapsed().as_millis() as f32/ 1000.0 ;
    println!("Done! Ran {} game(s) in {:.3} s", num_games, elapsed);
    print_summary(&s);
}

/// Play 1 round of CandyLand with the given number of players
fn play(p: &PlayerCount, deck: Option<Vec<Card>>) -> Stats {
    // Setup players
    let mut players: Vec<Player> = Vec::new();
    for i in 0..p.value() {
        players.push(Player {order: i + 1, space: 0, stuck: false});
    }

    // Setup deck
    let allow_reshuffle: bool;
    let mut d = if deck.is_none() {
        allow_reshuffle = true;
        make_deck()
    } else {
        allow_reshuffle = false;
        deck.unwrap()
    };
    if allow_reshuffle {
        d.shuffle(&mut thread_rng())
    }
    let deck_copy = d.clone(); // Cache off a deck for if/when we have to reshuffle
    d.reverse(); // Deck order is reversed so we can pull from the end

    // Let's do that Candy Land
    let board = get_board();
    let mut winner = 0;
    let mut num_turns = 0;
    let mut player_won = false;
    let mut reshuffle = 0;
    while !player_won {
        for mut p in &mut players {
            num_turns += 1;
            if p.stuck {
                if DEBUG {
                    println!("Player #{} is stuck. Skipping turn...", p.order);
                }
                p.stuck = false;
                continue;
            }
    
            let c = match d.pop() {
                Some(c) => c,
                None => {
                    reshuffle += 1;
                    d = deck_copy.clone();
                    if allow_reshuffle {                        
                        d.shuffle(&mut thread_rng());
                    }
                    d.reverse();
                    d.pop().unwrap()
                }
            };
            
            if reshuffle >= 3 && !allow_reshuffle {
                println!("Deck was an infinite loop: {}", write_deck(&deck_copy));
                return Stats{turns: 0, winner: 0, infinite: true};
            }

            move_player(&board, &mut p, c);
            if p.space == (board.len() - 1) {
                if DEBUG {
                    println!("Player {} won!", p.order);   
                }
                winner = p.order;
                player_won = true;
                break;
            }
        }
    }
    if DEBUG {
        println!("Done! {} turns", num_turns);
    }
    Stats{turns: num_turns, winner: winner, infinite: false}
}

fn make_deck() -> Vec<Card> {
    let mut c: Vec<Card> = Vec::new();
    for _ in 0..3 {
        c.push(Card {tile: Tile::Red, num: 1});
        c.push(Card {tile: Tile::Blue, num: 1});
        c.push(Card {tile: Tile::Purple, num: 1});
        c.push(Card {tile: Tile::Red, num: 2});
        c.push(Card {tile: Tile::Orange, num: 2});
        c.push(Card {tile: Tile::Yellow, num: 2});
        c.push(Card {tile: Tile::Green, num: 2});
        c.push(Card {tile: Tile::Blue, num: 2});
        c.push(Card {tile: Tile::Purple, num: 2});
    }

    for _ in 0..4 {
        c.push(Card {tile: Tile::Orange, num: 1});
        c.push(Card {tile: Tile::Yellow, num: 1});
        c.push(Card {tile: Tile::Green, num: 1});
    }

    c.push(Card {tile: Tile::IceCreamCone, num: 1});
    c.push(Card {tile: Tile::Gumdrop, num: 1});
    c.push(Card {tile: Tile::Lollipop, num: 1});
    c.push(Card {tile: Tile::Peppermint, num: 1});
    c.push(Card {tile: Tile::BonBon, num: 1});

    return c;
}

pub fn get_infinite_two_person_deck() -> Vec<Card> {
    let mut c: Vec<Card> = Vec::new();
    c.push(Card {tile: Tile::Peppermint, num: 1});
    c.push(Card {tile: Tile::Gumdrop, num: 1});
    c.push(Card {tile: Tile::Yellow, num: 1});
    c.push(Card {tile: Tile::Purple, num: 1});
    c.push(Card {tile: Tile::Blue, num: 1});
    c.push(Card {tile: Tile::Yellow, num: 1});
    c.push(Card {tile: Tile::Orange, num: 1});
    c.push(Card {tile: Tile::Blue, num: 1});
    c.push(Card {tile: Tile::Green, num: 1});
    c.push(Card {tile: Tile::Orange, num: 1});
    c.push(Card {tile: Tile::Red, num: 1});
    c.push(Card {tile: Tile::Green, num: 1});
    c.push(Card {tile: Tile::Purple, num: 1});
    c.push(Card {tile: Tile::Red, num: 2});
    c.push(Card {tile: Tile::Purple, num: 1});
    c.push(Card {tile: Tile::Yellow, num: 1});
    c.push(Card {tile: Tile::Yellow, num: 1});
    c.push(Card {tile: Tile::Blue, num: 1});
    c.push(Card {tile: Tile::Blue, num: 2});
    c.push(Card {tile: Tile::Orange, num: 2});
    c.push(Card {tile: Tile::Orange, num: 2});
    c.push(Card {tile: Tile::Red, num: 2});
    c.push(Card {tile: Tile::Purple, num: 2});
    c.push(Card {tile: Tile::Purple, num: 2});
    c.push(Card {tile: Tile::Yellow, num: 2});
    c.push(Card {tile: Tile::Yellow, num: 2});
    c.push(Card {tile: Tile::BonBon, num: 1});
    c.push(Card {tile: Tile::Orange, num: 1});
    c.push(Card {tile: Tile::Green, num: 2});
    c.push(Card {tile: Tile::Green, num: 2});
    c.push(Card {tile: Tile::Red, num: 1});
    c.push(Card {tile: Tile::Orange, num: 1});
    c.push(Card {tile: Tile::Purple, num: 2});
    c.push(Card {tile: Tile::Green, num: 2});
    c.push(Card {tile: Tile::Yellow, num: 2});
    c.push(Card {tile: Tile::Lollipop, num: 1});
    c.push(Card {tile: Tile::Green, num: 1});
    c.push(Card {tile: Tile::Blue, num: 2});
    c.push(Card {tile: Tile::Red, num: 1});
    c.push(Card {tile: Tile::Orange, num: 2});
    c.push(Card {tile: Tile::IceCreamCone, num: 1});
    c.push(Card {tile: Tile::Green, num: 1});
    c.push(Card {tile: Tile::Blue, num: 2});
    c.push(Card {tile: Tile::Red, num: 2});
    return c;
}

/// Decks are encoded with a single letter representing each card type.
/// For colored cards, lowercase indicates a one tile, uppercase two tiles.
/// Card to letter encodings are as follows:
///  - Red/Orange/Yellow/Green/Blue/Purple as R/O/Y/G/B/P
///  - Ice Cream Code as I
///  - Gumdrop as U
///  - Lollipop as L
///  - Peppermint as E
///  - Bon Bon as N
fn write_deck(cards: &Vec<Card>) -> String {
    let mut s = String::new();
    for c in cards {
        let mut temp = match c.tile {
            Tile::Red => String::from("r"),
            Tile::Orange => String::from("o"),
            Tile::Yellow => String::from("y"),
            Tile::Green => String::from("g"),
            Tile::Blue => String::from("b"),
            Tile::Purple => String::from("p"),
            Tile::IceCreamCone => String::from("I"),
            Tile::Gumdrop => String::from("U"),
            Tile::Lollipop => String::from("L"),
            Tile::Peppermint => String::from("E"),
            Tile::BonBon => String::from("N"),
            _ => {
                eprintln!("Invalid type {:?} found in card list!", c.tile);
                String::from("ERROR")
            },
        };

        if c.num > 1 {
            temp = temp.to_uppercase();
        }
        s.push_str(&temp);
    }
    return s;
}

fn print_summary(s: &Summary) {
    let len = s.turns.len();
    let mut avg_turns: f32 = 0.0;
    for t in &s.turns {
        avg_turns += *t as f32;
    }
    avg_turns /= len as f32;

    let med: f32;
    if len % 2 == 0 {
        med = ((s.turns.get(len / 2).unwrap() + s.turns.get(len / 2 + 1).unwrap()) as f32) / 2.0;
    }
    else {
        med = *s.turns.get(len / 2).unwrap() as f32;
    }
    println!("Average # turns: {}", avg_turns);
    println!("Median # turns: {}", med);
    let mut player = 1;
    for w in s.winners.iter() {
        println!("Player #{}: {}", player, w);
        player += 1;
    }
}