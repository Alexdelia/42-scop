mod graphic;
mod state;

fn main() {
    println!("Hello, world!");

    pollster::block_on(graphic::window::run());
}
