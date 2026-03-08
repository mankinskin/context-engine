mod items;
mod player;
mod enemy;
mod npc;
mod world;
mod combat;
mod game;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
