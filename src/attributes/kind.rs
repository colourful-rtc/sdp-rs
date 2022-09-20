use anyhow::{
    Result,
    anyhow
};

use std::{
    convert::TryFrom,
    fmt
};

/// This specifies the type of the multimedia conference.  Allowed values
/// are "broadcast", "meeting", "moderated", "test", and "H332".  These
/// values have implications for other options that are likely to be
/// appropriate:
/// 
/// *  When "a=type:broadcast" is specified, "a=recvonly" is probably
/// appropriate for those connecting.
/// 
/// *  When "a=type:meeting" is specified, "a=sendrecv" is likely to be
/// appropriate.
/// 
/// *  "a=type:moderated" suggests the use of a floor control tool and
/// that the media tools be started so as to mute new sites joining
/// the multimedia conference.
/// 
/// *  Specifying "a=type:H332" indicates that this loosely coupled
/// session is part of an H.332 session as defined in the ITU H.332
/// specification [ITU.H332.1998](https://datatracker.ietf.org/doc/
/// html/rfc8866#ref-ITU.H332.1998). Media tools should be started 
/// using "a=recvonly".
/// 
/// *  Specifying "a=type:test" is suggested as a hint that, unless
/// explicitly requested otherwise, receivers can safely avoid
/// displaying this session description to users.
#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    Broadcast,
    Meeting,
    Moderated,
    Test,
    H332
}

impl fmt::Display for Kind {
    /// # Unit Test
    ///
    /// ```
    /// use sdp::attributes::*;
    ///
    /// assert_eq!(format!("{}", Kind::Broadcast), "broadcast");
    /// assert_eq!(format!("{}", Kind::Meeting), "meeting");
    /// assert_eq!(format!("{}", Kind::Moderated), "moderated");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Broadcast =>  "broadcast",
            Self::Meeting =>    "meeting",
            Self::Moderated =>  "moderated",
            Self::Test =>       "test",
            Self::H332 =>       "H332",
        })
    }
}

impl<'a> TryFrom<&'a str> for Kind {
    type Error = anyhow::Error;
    /// # Unit Test
    ///
    /// ```
    /// use sdp::attributes::*;
    /// use std::convert::*;
    ///
    /// assert_eq!(Kind::try_from("broadcast").unwrap(), Kind::Broadcast);
    /// assert_eq!(Kind::try_from("meeting").unwrap(), Kind::Meeting);
    /// assert_eq!(Kind::try_from("moderated").unwrap(), Kind::Moderated);
    /// assert!(Kind::try_from("av1x").is_err());
    /// ```
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "broadcast" =>  Ok(Self::Broadcast),
            "meeting" =>    Ok(Self::Meeting),
            "moderated" =>  Ok(Self::Moderated),
            "test" =>       Ok(Self::Test),
            "H332" =>       Ok(Self::H332),
            _ => Err(anyhow!("invalid type!"))
        }
    }
}
