use const_format::concatcp;
use git_version::git_version;

pub const USER_AGENT: &str = concatcp!(env!("CARGO_PKG_NAME"), "/", VERSION);
pub const VERSION: &str = concatcp!(
    env!("CARGO_PKG_VERSION"),
    "+",
    git_version!(fallback = "unknown")
);
