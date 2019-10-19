use std::io;
use std::path::PathBuf;

use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("malformed trashinfo path name '{}'", path.display()))]
    InvalidTrashInfoPath { path: PathBuf },

    #[snafu(display("failed to read trash info dir: {}", source))]
    ReadTrashInfoDir { source: io::Error },

    #[snafu(display("failed to write file '{}': {}", path.display(), source))]
    WriteTrashInfo { path: PathBuf, source: io::Error },

    #[snafu(display("failed to read file '{}': {}", path.display(), source))]
    ReadTrashInfo { path: PathBuf, source: io::Error },

    #[snafu(display("failed to move file '{}': {}", path.display(), source))]
    MoveFile { path: PathBuf, source: io::Error },

    #[snafu(display("failed to remove file '{}': {}", path.display(), source))]
    RemoveFile { path: PathBuf, source: io::Error },

    #[snafu(display("file does not exist '{}'", path.display()))]
    InvalidPath { path: PathBuf },

    #[snafu(display("cannot trash trash-can '{}'", path.display()))]
    TrashingTrashCan { path: PathBuf },

    #[snafu(display("failed to parse TrashInfo file '{}': {}", path.display(), source))]
    ParseTrashInfo { path: PathBuf, source: io::Error },
}
