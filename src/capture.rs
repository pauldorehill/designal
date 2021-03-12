use std::{fs::{File, OpenOptions}, io::Read, path::PathBuf};

/// This is used to check if the files should be generated from the derive calls
pub(crate) const FILE_MESSAGE: &[u8] = "// Crunch\n".as_bytes();

/// Finds the output path & creates it if required
pub(crate) fn output_path(make_dir_path: bool) -> Option<PathBuf> {
    match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(root) => {
            let mut path = PathBuf::from(root);
            path.push("target/designal");
            if make_dir_path {
                std::fs::create_dir_all(&path).unwrap();
            }
            path.push("out.rs");
            Some(path)
        }
        Err(_) => None,
    }
}

pub(crate) fn edit_file<F>(mut action: F)
where
    F: FnMut(File),
{
    if let Some(path) = output_path(false) {
        if let Ok(mut file) = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&path)
        {
            let mut buf = [0; FILE_MESSAGE.len()];
            match file.read_exact(&mut buf) {
                Ok(_) => {
                    if buf == FILE_MESSAGE {
                        action(file)
                    }
                }
                Err(_) => {
                    if let Some(path) = path.parent() {
                        std::fs::remove_dir_all(path).unwrap_or(());
                    }
                }
            }
        }
    }
}
