mod game;

use game::player::Player;
use game::Game;

fn main() {
    let mut game = Game::new();

    game.play();
}
