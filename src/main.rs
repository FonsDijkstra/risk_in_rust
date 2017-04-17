mod game;
mod board;
mod player;

fn main() {
    println!("Risk in Rust");

    let mut game = game::Game::new();
    game.add_player(player::Color::Red);
    game.add_player(player::Color::Green);
    game.add_player(player::Color::Yellow);
    game.start();

    println!("{:?}", game);
}

