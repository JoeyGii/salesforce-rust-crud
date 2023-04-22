use serde::Deserialize;

use std::fmt;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: u16, error_message: String) -> CustomError {
        CustomError {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

pub struct UserError {
    pub error_type: UserErrorTypes,
    pub is_error: bool,
}
impl Default for UserError {
    fn default() -> Self {
        UserError {
            error_type: UserErrorTypes::GoodToGo,
            is_error: false,
        }
    }
}
impl UserError {
    pub fn get_error_message(&self) -> String {
        match self.error_type {
            UserErrorTypes::GoodToGo => String::new(),
            UserErrorTypes::WrongPassword => String::from("password doesn't match the username :("),
            UserErrorTypes::UsernameHasSpaces => {
                String::from("don't use spaces in your username!!")
            }
            UserErrorTypes::UsernameIsBlank => String::from("Your username is blank!"),
            UserErrorTypes::InvalidConnection => {
                String::from("Use commas between connections, and make sure you got the name right")
            }
            UserErrorTypes::MessageExcedesLength => {
                String::from("Messages must be no more than 70 characters")
            }
        }
    }
}
pub enum UserErrorTypes {
    GoodToGo,
    UsernameHasSpaces,
    UsernameIsBlank,
    WrongPassword,
    InvalidConnection,
    MessageExcedesLength,
}
