use std::io;
use std::path::PathBuf;

use failure::Error;

#[derive(Debug, Fail)]
#[fail(display = "Failed to access file {:?}: {}", path, inner)]
pub struct AccessError {
    pub path: PathBuf,
    #[cause]
    pub inner: io::Error,
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub(crate) fn log_error(e: &Error) {
    error!("error: {}", e.cause());
    for cause in e.causes() {
        error!("caused by: {}", cause);
    }

    debug!("backtrace: {}", e.backtrace());
}