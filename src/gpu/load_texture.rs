use crate::parse::load_dir;
use crate::setting::TEXTURE_PATH;

use ansi::abbrev::{B, D, M, N_C, R};

use std::io::{stdout, Write};
use std::path::{Path, PathBuf};

enum LoadError {
    Image(image::ImageError),
    Texture(glium::texture::TextureCreationError),
}

pub fn load_texture(display: &glium::Display) -> Vec<glium::texture::SrgbTexture2d> {
    let Ok(dir) = load_dir(TEXTURE_PATH) else {
        return vec![empty_texture(display)];
    };

    let texture = filter_texture(dir_to_texture(display, dir));

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

fn dir_to_texture(
    display: &glium::Display,
    dir: Vec<PathBuf>,
) -> Vec<(
    PathBuf,
    std::result::Result<glium::texture::SrgbTexture2d, LoadError>,
)> {
    let mut texture: Vec<(
        PathBuf,
        std::result::Result<glium::texture::SrgbTexture2d, LoadError>,
    )> = Vec::new();
    let mut stdout = stdout();

    print!("loading {B}{M}{}{D} texture\t{B}", dir.len());
    stdout.flush().expect("failed to flush stdout");

    for path in dir {
        match path_to_texture(display, &path) {
            Ok(t) => {
                print!(".");
                stdout.flush().expect("failed to flush stdout");
                texture.push((path, Ok(t)))
            }
            Err(e) => {
                print!("{R}x{N_C}");
                stdout.flush().expect("failed to flush stdout");
                texture.push((path, Err(e)))
            }
        }
    }

    println!("{D}");

    texture
}

fn path_to_texture(
    display: &glium::Display,
    path: &Path,
) -> std::result::Result<glium::texture::SrgbTexture2d, LoadError> {
    let image = image::open(path)
        .map_err(|e| {
            eprintln!("\nfailed to open texture path '{}'\n{e}", path.display());
            LoadError::Image(e)
        })?
        .into_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::SrgbTexture2d::new(display, image).map_err(|e| {
        eprintln!(
            "\nfailed to create texture from image '{}'\n{e}",
            path.display(),
        );
        LoadError::Texture(e)
    })
}

fn filter_texture(
    texture: Vec<(
        PathBuf,
        std::result::Result<glium::texture::SrgbTexture2d, LoadError>,
    )>,
) -> Vec<glium::texture::SrgbTexture2d> {
    texture
        .into_iter()
        .filter_map(|entry| match entry.1 {
            Ok(t) => Some(t),
            Err(e) => {
                eprintln!(
                    "failed to load texture '{B}{M}{tex}{D}'\n{err}",
                    tex = entry.0.display(),
                    err = match e {
                        LoadError::Image(e) => format!("{:?}", e),
                        LoadError::Texture(e) => format!("{:?}", e),
                    }
                );
                None
            }
        })
        .collect()
}
