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
