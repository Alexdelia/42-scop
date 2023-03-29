use crate::setting::Setting;

use yahmrslib::hmerr::Result;

use glium::glutin::{self, window::Icon};

use std::path::Path;

pub struct Env {
    pub display: glium::Display,
    pub gpu: crate::gpu::Gpu,
    pub setting: Setting,
}

impl Env {
    pub fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Result<Self> {
        let setting = Setting::default();
        let display = glium::Display::new(
            glutin::window::WindowBuilder::new()
                .with_title(setting.title)
                .with_transparent(true)
                .with_window_icon(load_icon(setting.icon)),
            glutin::ContextBuilder::new(),
            &event_loop,
        )?;
        {
            use glium::Surface;
            let mut frame = display.draw();
            frame.clear_color(
                setting.bg_color.r,
                setting.bg_color.g,
                setting.bg_color.b,
                setting.bg_color.a,
            );
            frame.finish().unwrap();
        }
        let gpu = crate::gpu::Gpu::new(&display)?;

        Ok(Self {
            display,
            gpu,
            setting,
        })
    }
}

fn load_icon(path: &Path) -> Option<Icon> {
    let Ok(image) = image::open(path) else {
		eprintln!("failed to open icon path '{}'", path.display());
		return None;
	};
    let image = image.into_rgba8();
    let (w, h) = image.dimensions();
    if let Ok(icon) = Icon::from_rgba(image.into_raw(), w, h) {
        Some(icon)
    } else {
        eprintln!("failed to convert icon '{}' image to rgba", path.display());
        None
    }
}
