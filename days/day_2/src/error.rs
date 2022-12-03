use std::fmt::Debug;

pub type MyError = String;

pub type Result<T, E = MyError> = std::result::Result<T, E>;

pub fn error_to_string<E: Debug>(e: E) -> String {
    format!("Error: {:?}", e)
}
