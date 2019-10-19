use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use path_clean::PathClean;

/// Creates any necessary destination directories.
/// Adds a number to the end of the destionation path if there are any path conflicts.
/// Treats the destination as a file and does not move the file into the directory if the destination is a directory.
pub fn move_file(from: &Path, to: &Path) -> io::Result<PathBuf>
where
{
    let to_dir = to.parent().expect("path is valid destination").to_owned();
    let to_filename = to
        .file_name()
        .expect("path is clean")
        .to_string_lossy()
        .to_string();

    fs::create_dir_all(&to_dir)?;

    let mut count = 1;
    let mut to = to.to_path_buf();
    while to.exists() {
        to = to.with_file_name(format!("{}_{}", to_filename, count));
        count += 1;
    }

    fs::rename(&from, &to)?;

    Ok(to)
}

/// Propogates errors from `std::env::current_dir`.
/// Must be used on all paths accepted by public functions since relative paths are generally prone to issues.
/// Example: `PathBuf::from(".").file_name() == None`, `PathBuf::from(".").starts_with("/") == False`
pub fn absolute_path<P>(path: P) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let pathbuf = if path.is_absolute() {
        path.to_path_buf()
    } else {
        if path == PathBuf::from("") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path cannot be empty",
            ));
        } else {
            env::current_dir()?.join(path)
        }
    }
    // cleans up the path since we don't want something like `/foo/bar/..`
    .clean();

    Ok(pathbuf)
}

pub trait AbsolutePath {
    fn absolute_path(&self) -> io::Result<PathBuf>;
}

impl AbsolutePath for PathBuf {
    fn absolute_path(&self) -> io::Result<PathBuf> {
        absolute_path(&self)
    }
}

impl AbsolutePath for Path {
    fn absolute_path(&self) -> io::Result<PathBuf> {
        absolute_path(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename_file_handle_conflicts() {
        use std::fs::File;

        let folder = PathBuf::from("test1");
        fs::create_dir_all(&folder);
        let file = folder.join("file");
        let file1 = folder.join("foo");
        File::create(&file);
        File::create(&file1);
        assert!(&file.exists());
        assert_eq!(move_file(&file, &file1).unwrap(), folder.join("foo_1"));
        assert!(!&file.exists());
        File::create(&file);
        assert!(move_file(&file, &file.join("asdf")).is_err());
        assert_eq!(
            move_file(&file, &PathBuf::from(format!("{}asdf", file.display(),))).unwrap(),
            folder.join("fileasdf")
        );

        fs::remove_dir_all(folder).unwrap();
    }
}
