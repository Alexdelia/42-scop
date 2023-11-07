use std::path::Path;

pub fn load_dir(dir: impl AsRef<Path>) -> std::io::Result<Vec<std::path::PathBuf>> {
    let path = dir.as_ref();
    let mut file: Vec<std::path::PathBuf> = std::fs::read_dir(path)
        .map_err(|e| {
            eprintln!("failed to read directory '{}'\n{e}", path.display());
            e
        })?
        .filter_map(|e| {
            e.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();
    file.sort();
    Ok(file)
}
