use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::board::*;

mod board;

const DEBUG: bool = false;

struct Stats {
    turns: u32,
    winner: u32,
}

pub struct Summary {
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct Card {
    tile: Tile,
    num: u32,
}

pub fn run_multiple(p: PlayerCount, num_games: u32) -> Summary {
    let mut s = match p {
        PlayerCount::Two => Summary{turns: Vec::new(), winners: vec![0; 2]},
        PlayerCount::Three => Summary{turns: Vec::new(), winners: vec![0; 3]},
        PlayerCount::Four => Summary{turns: Vec::new(), winners: vec![0; 4]},
    };

    for _ in 0..num_games {
        let stats = run(&p);
        s.turns.push(stats.turns);
        *s.winners.get_mut((stats.winner - 1) as usize).unwrap() += 1;
    }
    return s;
}

fn run(p: &PlayerCount) -> Stats {
    // Setup players
    let mut players: Vec<Player> = Vec::new();
    for i in 0..p.value() {
        players.push(Player {order: i + 1, space: 0, stuck: false});
    }

    let board = get_board();
    let mut deck: VecDeque<Card> = make_deck();
    let mut deck_string = write_deck(&deck); // Cache off deck just in case we're logging

    // Let's do that Candy Land
    let mut winner = 0;
    let mut num_turns = 0;
    let mut player_won = false;
    let mut reshuffle = 0;
    while !player_won {
        for mut p in &mut players {
            num_turns += 1;
            if p.stuck {
                p.stuck = false;
                continue;
            }
    
            let c = match deck.pop_front() {
                Some(c) => c,
                None => {
                    if DEBUG {
                        println!("Current deck required a reshuffle: {} (#{})", deck_string, reshuffle);
                    }
                    reshuffle += 1;
                    deck = make_deck();
                    deck_string = write_deck(&deck);
                    deck.pop_front().unwrap()
                }
            };
    
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
    Stats{turns: num_turns, winner: winner}
}

fn make_deck() -> VecDeque<Card> {
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

    c.shuffle(&mut thread_rng());
    return VecDeque::from(c);
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
fn write_deck(cards: &VecDeque<Card>) -> String {
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

pub fn print_summary(s: &Summary) {
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