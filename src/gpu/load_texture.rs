use crate::parse::load_dir;
use crate::setting::TEXTURE_PATH;

enum LoadError {
    Image(image::ImageError),
    Texture(glium::texture::TextureCreationError),
}

pub fn load_texture(display: &glium::Display) -> Vec<glium::texture::SrgbTexture2d> {
    let Ok(dir) = load_dir(TEXTURE_PATH) else {
		return vec![empty_texture(display)];
	};
    let mut texture: Vec<(
        std::path::PathBuf,
        std::result::Result<glium::texture::SrgbTexture2d, LoadError>,
    )> = Vec::new();

    println!("loading {} texture", dir.len());
    for path in dir {
        match path_to_texture(display, &path) {
            Ok(t) => {
                print!(".");
                texture.push((path, Ok(t)))
            }
            Err(e) => {
                print!("x");
                texture.push((path, Err(e)))
            }
        }
    }

    let texture = filter_texture(texture);

    if texture.is_empty() {
        vec![empty_texture(display)]
    } else {
        texture
    }
}

fn empty_texture(display: &glium::Display) -> glium::texture::SrgbTexture2d {
    glium::texture::SrgbTexture2d::empty(display, 1, 1)
        .map_err(|e| {
            eprintln!("failed to create empty texture\n{e}");
            e
        })
        .unwrap()
}

fn path_to_texture(
    display: &glium::Display,
    path: &std::path::Path,
) -> std::result::Result<glium::texture::SrgbTexture2d, LoadError> {
    let image = image::open(path)
        .map_err(|e| {
            eprintln!("failed to open texture path '{}'\n{e}", path.display());
            LoadError::Image(e)
        })?
        .into_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::SrgbTexture2d::new(display, image).map_err(|e| {
        eprintln!(
            "failed to create texture from image '{}'\n{e}",
            path.display(),
        );
        LoadError::Texture(e)
    })
}

fn filter_texture(
    texture: Vec<(
        std::path::PathBuf,
        std::result::Result<glium::texture::SrgbTexture2d, LoadError>,
    )>,
) -> Vec<glium::texture::SrgbTexture2d> {
    texture
        .into_iter()
        .filter_map(|entry| match entry.1 {
            Ok(t) => Some(t),
            Err(e) => {
                eprintln!(
                    "failed to load texture '{}'\n{}",
                    entry.0.display(),
                    match e {
                        LoadError::Image(e) => format!("{:?}", e),
                        LoadError::Texture(e) => format!("{:?}", e),
                    }
                );
                None
            }
        })
        .collect()
}
