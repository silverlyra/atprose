use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

/// An [AT protocol handle][handle].
///
/// [handle]: https://atproto.com/specs/handle
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub struct Handle<T = String>(T);

impl<T> Handle<T>
where
    T: Deref<Target = str>,
{
    pub fn new<V>(value: V) -> Self
    where
        T: From<V>,
        V: AsRef<str>,
    {
        Self::parse(value).expect("invalid handle")
    }

    pub fn parse<V>(value: V) -> Result<Self, InvalidHandle>
    where
        T: From<V>,
        V: AsRef<str>,
    {
        match validate_handle(value.as_ref()) {
            Ok(_) => Ok(Self(value.into())),
            Err(err) => Err(err),
        }
    }
}

impl<T> Handle<T> {
    const MAX_LENGTH: usize = 0x100 - 3;
    const MAX_SEGMENT_LENGTH: usize = 0x40 - 1;

    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }
}

impl FromStr for Handle<String> {
    type Err = InvalidHandle;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl<T: Display> Display for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(thiserror::Error, PartialEq, Eq, Clone, Debug)]
pub enum InvalidHandle {
    #[error("empty AT handle")]
    Empty,
    #[error("AT handle too long")]
    Length,
    #[error("AT handle domain disallowed")]
    Domain,
    #[error("empty AT handle segment")]
    SegmentEmpty,
    #[error("AT handle segment too long")]
    SegmentLength,
    #[error("invalid character in AT handle: {0:?}")]
    Character(char),
}

fn validate_handle(value: &str) -> Result<(), InvalidHandle> {
    if value.is_empty() {
        return Err(InvalidHandle::Empty);
    } else if value.len() > Handle::<&str>::MAX_LENGTH {
        return Err(InvalidHandle::Length);
    }

    let segments: Vec<_> = value.split('.').collect();
    if segments.len() < 2 {
        return Err(InvalidHandle::Domain);
    }
    let ls = segments.len() - 1;

    match segments.last().copied().unwrap() {
        "alt" | "arpa" | "example" | "internal" | "invalid" | "local" | "localhost" | "onion" => {
            return Err(InvalidHandle::Domain)
        }
        _ => {}
    }

    for (i, segment) in segments.into_iter().enumerate() {
        if segment.is_empty() {
            return Err(InvalidHandle::SegmentEmpty);
        } else if segment.len() > Handle::<&str>::MAX_SEGMENT_LENGTH {
            return Err(InvalidHandle::SegmentLength);
        }

        let lc = segment.len() - 1;
        for (j, c) in segment.chars().enumerate() {
            match (i, j, c) {
                (_, _, 'a'..='z' | 'A'..='Z') => {}
                (i, j, '0'..='9') if i < ls || j > 0 => {}
                (_, j, '-') if j > 0 && j < lc => {}
                (_, _, c) => return Err(InvalidHandle::Character(c)),
            };
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{validate_handle, InvalidHandle};

    #[test]
    fn test_validate_handle() {
        use InvalidHandle::*;

        let valid = [
            "jay.bsky.social",
            "8.cn",
            "name.t--t",
            "XX.LCS.MIT.EDU",
            "a.co",
            "xn--notarealidn.com",
            "xn--fiqa61au8b7zsevnm8ak20mc4a87e.xn--fiqs8s",
            "xn--ls8h.test",
            "example.t",
        ];

        let invalid = [
            ("", Empty),
            ("jo@hn.test", Character('@')),
            ("💩.test", Character('💩')),
            (
                "me.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl.com",
                SegmentLength,
            ),
            ("john..test", SegmentEmpty),
            ("xn--bcher-.tld", Character('-')),
            ("john.0", Character('0')),
            ("cn.8", Character('8')),
            ("www.masełkowski.pl.com", Character('ł')),
            ("org", Domain),
            ("name.org.", SegmentEmpty),
            (
                "2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion",
                Domain,
            ),
            ("laptop.local", Domain),
            ("blah.arpa", Domain),
        ];

        for value in valid {
            assert_eq!(Ok(()), validate_handle(value));
        }

        for (value, expected) in invalid {
            assert_eq!(Err(expected), validate_handle(value));
        }
    }
}
