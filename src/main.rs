use std::time::Instant;
use candy_land::*;

const PLAYER_COUNT:PlayerCount = PlayerCount::Two;
const NUM_GAMES: u32 = 1000;

fn main() {
    let now = Instant::now();
    let s = run_multiple(PLAYER_COUNT, NUM_GAMES);
    let elapsed = now.elapsed().as_millis() as f32/ 1000.0 ;
    println!("Done! Ran {} game(s) in {:.3} s", NUM_GAMES, elapsed);
    print_summary(&s);
}