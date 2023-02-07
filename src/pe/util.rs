/// A macro to return `Ok(None)` if the expression is `None`.
macro_rules! try_option {
  ($e:expr) => {
    match $e {
      Some(e) => e,
      None => return Ok(None),
    }
  };
}

pub(crate) use try_option;
