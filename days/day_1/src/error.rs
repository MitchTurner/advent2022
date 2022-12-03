pub type MyError = String;

pub type Result<T, E=MyError> = std::result::Result<T, E>;

