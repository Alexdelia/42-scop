mod event;
mod gpu;
mod setting;
mod state;
mod window;

fn main() {
    println!("Hello, world!");

    pollster::block_on(window::run());
}
