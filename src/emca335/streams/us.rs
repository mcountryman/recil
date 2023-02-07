//! `#US` stream data.

use crate::{emca335::header::StreamHeader, error::Error};

/// An index into the [UserStringsHeap].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UserStringId(u32);

/// Contains UTF16 encoded strings prefixed by a variable sized length, indexable by a [UserStringId].  
///
/// Can contain garbage data, as long as any part that is reachable from any of the tables contains
/// a valid 'blob'.
#[derive(Debug, Default)]
pub struct UserStringsHeap<'a>(pub(crate) &'a [u8]);

impl<'a> UserStringsHeap<'a> {
  /// Parses the `#US` stream from the given buffer.
  pub fn parse(from: &'a [u8], header: &StreamHeader<'a>) -> Result<Self, Error> {
    Ok(Self(header.data(from)?))
  }
}
