use std::path::Path;

pub fn load_dir(path: &Path) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    Ok(std::fs::read_dir(path)
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
        .collect())
}
