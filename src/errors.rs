use std::error;
use std::fmt;
use strum::EnumMessage;
use strum_macros;

#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum DBError {
    #[strum(message = "Failed getting db connection")]
    ConnectionFailed,

    #[strum(message = "user not found")]
    UserNotFound,

    #[strum(message = "no files found")]
    NoFiles,

    #[strum(message = "Failed to insert new file")]
    FileCreationFailed,
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for DBError {
    fn description(&self) -> &str {
        self.get_message().unwrap()
    }
}

#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum AuthError {
    #[strum(message = "Your login details are incorrect.")]
    LoginError,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for AuthError {
    fn description(&self) -> &str {
        self.get_message().unwrap()
    }
}

#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum FileError {
    #[strum(message = "File upload failed.")]
    UploadFailed,

    #[strum(message = "File upload failed.")]
    DownloadFailed,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for FileError {
    fn description(&self) -> &str {
        self.get_message().unwrap()
    }
}
