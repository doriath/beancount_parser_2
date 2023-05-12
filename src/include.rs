use std::path::{Path, PathBuf};

use nom::{
    bytes::complete::tag,
    character::complete::space1,
    combinator::map,
    sequence::{preceded, tuple},
};

use crate::{string::string, IResult};

/// Include directive
#[derive(Clone, Debug)]
pub struct Include {
    path: PathBuf,
}

impl Include {
    /// Path to include
    #[must_use]
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

pub(crate) fn include(input: crate::Input<'_>) -> IResult<'_, Include> {
    map(preceded(tuple((tag("include"), space1)), string), |path| {
        Include { path: path.into() }
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::combinator::all_consuming;

    #[test]
    fn valid_include_directive() {
        let (_, inc) = include(r#"include "abc.beancount""#).unwrap();
        assert_eq!(inc.path().to_str(), Some("abc.beancount"));
    }

    #[rstest]
    fn invalid(#[values("include", r#"include "a" "b""#)] input: &str) {
        assert!(matches!(
            all_consuming(include)(input),
            Err(nom::Err::Error(_))
        ));
    }
}
