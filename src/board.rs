use crate::*;
use std::cell::Cell;

pub struct Space {
    tile: Tile,
    shortcut: Option<usize>,
    sticky: bool,
    player: Cell<bool>,
}

pub fn get_board() -> Vec<Space> {
    vec![
        Space {tile: Tile::Start,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: Some(36), sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: Some(18), sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Peppermint,   shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: true,  player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Gumdrop,      shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::BonBon,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Lollipop,     shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: true,  player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::IceCreamCone, shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Red,          shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Purple,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Yellow,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Blue,         shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Orange,       shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::Green,        shortcut: None,     sticky: false, player: Cell::new(false)},
        Space {tile: Tile::End,          shortcut: None,     sticky: false, player: Cell::new(false)},
    ]
}

pub fn move_player(board: &Vec<Space>, player: &mut Player, card: Card) {
    if DEBUG {
        println!("Moving Player {} from space #{}: {} {:?}", player.order, player.space, card.num, card.tile);
    }
    
    // Mark the current space the player is on as free, then find the next matching space
    // If no match is found, we've reached the end
    let mut s: usize = player.space;
    board.get(s).unwrap().player.set(false);
    match card.tile {
        Tile::Peppermint => s = board.iter().position(|q| q.tile == Tile::Peppermint).unwrap(),
        Tile::Gumdrop => s = board.iter().position(|q| q.tile == Tile::Gumdrop).unwrap(),
        Tile::BonBon => s = board.iter().position(|q| q.tile == Tile::BonBon).unwrap(),
        Tile::Lollipop => s = board.iter().position(|q| q.tile == Tile::Lollipop).unwrap(),
        Tile::IceCreamCone => s = board.iter().position(|q| q.tile == Tile::IceCreamCone).unwrap(),
        _ => { // Otherwise, just advance the player forward
            let mut num = 0;
            loop {
                num += 1;
                match board.iter().skip(s + 1).position(|q| q.tile == card.tile) { // Skip 1 past the current space to avoid returning the same value
                    Some(q) => {
                        s += q + 1; // Add the returned position value (+1) as it is relative to the amount skipped
                        if num >= card.num && !board.get(s).unwrap().player.get() {
                            break;
                        }
                    } 
                    None => {
                        s = board.iter().position(|q| q.tile == Tile::End).unwrap(); // There's no next one, so the player has reached the end
                        break;
                    } 
                }
            }
        }
    }

    let space = board.get(s).unwrap();
    space.player.set(true);
    player.space = s;
    player.stuck = space.sticky;

    if space.shortcut.is_some() {
        if DEBUG {
            println!("Space #{} is a shortcut to #{}", s, space.shortcut.unwrap());
        }
        player.space = space.shortcut.unwrap();
    }

    if DEBUG {
        println!("Player at space #{}", player.space);
    }
}