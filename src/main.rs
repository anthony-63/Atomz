use macroquad::prelude::*;
use world::World;

pub mod atom;
pub mod world;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

fn conf() -> Conf {
    Conf {
        window_title: "Atomz | Atom simulation".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    let mut world = World::new(WIDTH, HEIGHT);
    world.new_group("yellow", 100, YELLOW);
    world.new_group("red", 100, RED);
    world.new_group("green", 100, GREEN);
    loop {
        clear_background(BLACK);
        world.draw();
        world.interact("green", "green", -0.32);
        world.interact("green", "red", -0.17);
        world.interact("green", "yellow", 0.34);
        world.interact("red", "red", -0.10);
        world.interact("red", "green", -0.34);
        world.interact("yellow", "yellow", 0.15);
        world.interact("yellow", "green", -0.20);

        next_frame().await
    }
}
