mod combat;
mod enemy;
mod game;
mod items;
mod map_render;
mod npc;
mod player;
mod world;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
