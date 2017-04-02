mod game;
mod board;
mod player;

fn main() {
    println!("Risk in Rust");

    let mut game = game::Game::new();
    game.add_player(player::Color::Black);
    game.add_player(player::Color::Red);
    game.start();

    println!("{:?}", game);
}

