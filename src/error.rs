use core::{fmt, str::Utf8Error};
#[cfg(feature = "std")]
use std::{error, io};

#[derive(Debug)]
/// A CIL error
pub enum Error {
  /// An IO based error
  #[cfg(feature = "std")]
  IO(io::Error),
  /// A UTF-8 error
  Utf8(Utf8Error),
  /// The entity is malformed.
  Malformed(&'static str),
  /// A table row id was out of bounds.
  BadRowId(u32),
  /// The requested length to read/write at is invalid
  BadLength(u32),
  /// The requested offset to read/write at is invalid
  BadOffset(u32),
  /// Bad magic was performed.  Try saying the magic word.
  BadMagic(&'static str),
  /// A GUID index was out of bounds.
  BadGuidId(u32),
  /// A string index was out of bounds.
  BadStringId(u32),
  Scroll(scroll::Error),
  Goblin(goblin::error::Error),
}

#[cfg(feature = "std")]
impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match *self {
      Self::IO(ref io) => Some(io),
      _ => None,
    }
  }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Self::IO(err)
  }
}

impl From<scroll::Error> for Error {
  fn from(err: scroll::Error) -> Self {
    Self::Scroll(err)
  }
}

impl From<goblin::error::Error> for Error {
  fn from(err: goblin::error::Error) -> Self {
    Self::Goblin(err)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      #[cfg(feature = "std")]
      Self::IO(ref err) => write!(fmt, "{err}"),
      Self::Utf8(ref err) => write!(fmt, "{err}"),
      Self::Malformed(item) => write!(fmt, "Malformed {item}"),
      Self::BadRowId(offset) => write!(fmt, "Bad row id `{offset}`"),
      Self::BadLength(offset) => write!(fmt, "Bad length `{offset}`"),
      Self::BadOffset(offset) => write!(fmt, "Bad offset `{offset}`"),
      Self::BadMagic(item) => write!(fmt, "Bad magic for {item}"),
      Self::BadGuidId(offset) => write!(fmt, "Bad guid id `{offset}`"),
      Self::BadStringId(offset) => write!(fmt, "Bad string id `{offset}`"),
      Self::Scroll(err) => write!(fmt, "{err}"),
      Self::Goblin(err) => write!(fmt, "{err}"),
    }
  }
}
