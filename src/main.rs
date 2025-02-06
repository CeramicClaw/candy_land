use candy_land::*;

const PLAYER_COUNT:PlayerCount = PlayerCount::Two;
const NUM_GAMES: u32 = 10000000;

fn main() {
    calculate(PLAYER_COUNT, NUM_GAMES, None);
    //calculate(PLAYER_COUNT, NUM_GAMES, Some(get_infinite_two_person_deck()));
}