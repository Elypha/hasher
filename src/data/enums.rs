use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
pub enum UserAction {
    Check,
    Size,
    XXH3,
    // SHA256,
    // MD5,
}

impl fmt::Display for UserAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserAction::Size => write!(f, "size"),
            UserAction::Check => write!(f, "check"),
            UserAction::XXH3 => write!(f, "xxh3"),
            // UserAction::SHA256 => write!(f, "SHA256"),
            // UserAction::MD5 => write!(f, "MD5"),
        }
    }
}

impl FromStr for UserAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "size" => Ok(UserAction::Size),
            "check" => Ok(UserAction::Check),
            "xxh3" => Ok(UserAction::XXH3),
            // "sha256" => Ok(UserAction::SHA256),
            // "md5" => Ok(UserAction::MD5),
            _ => Err(format!("Unsupported action: {}", s)),
        }
    }
}
