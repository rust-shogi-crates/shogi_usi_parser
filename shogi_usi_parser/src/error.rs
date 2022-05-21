use core::fmt::Display;

/// An error that can occur while parsing.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    /// The parser failed to parse `s[from..to]`.
    InvalidInput {
        from: usize,
        to: usize,
        description: &'static str,
    },
    /// After parsing the input, `s[from..]` was left unread.
    Extra { from: usize },
    /// Parsing was successful, but we got an invalid [`Position`][shogi_core::Position].
    InvalidPosition,
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Error::InvalidInput {
                from,
                to,
                description,
            } => write!(
                f,
                "Invalid input: failed to parse s[{}..{}]: {}",
                from, to, description,
            ),
            Error::Extra { from } => write!(f, "Extra input: s[{}..] was left unread", from),
            Error::InvalidPosition => f.write_str("Invalid position"),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

/// Custom result type for this crate.
pub type Result<T> = core::result::Result<T, Error>;
