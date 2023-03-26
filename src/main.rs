mod env;
mod event;
mod render;
mod setting;
mod window;

use yahmrslib::hmerr::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    window::run()?;

    Ok(())
}
