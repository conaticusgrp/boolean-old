use std::result::Result;

pub type CommandResult = Result<Response, String>;

pub enum Response {
    // Don't respond (the command will do it)
    Ignore,
    // Respond without a message
    Success,
    // Respond with a message (bool: ephemeral)
    Ok(String, bool),
    // User error (tell the user something went wrong that they can fix)
    Warning(String),
}

impl Response {
    pub fn warning(msg: &str) -> Result<Self, String> {
        Ok(Response::Warning(msg.to_string()))
    }

    pub fn ok(msg: &str, ephemeral: bool) -> Result<Self, String> {
        Ok(Response::Ok(msg.to_string(), ephemeral))
    }

    pub fn success() -> Result<Self, String> {
        Ok(Response::Success)
    }

    pub fn ignore() -> Result<Self, String> {
        Ok(Response::Ignore)
    }

    pub fn err<T: ToString>(msg: T) -> Result<Self, String> {
        Err(msg.to_string())
    }
}
