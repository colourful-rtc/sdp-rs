use crate::util::tuple2_from_split;
use std::convert::TryFrom;
use anyhow::Result;

/// attribute name (as it will appear in SDP): extmap
/// 
/// long-form attribute name in English: generic header extension map
/// definition
/// 
/// type of attribute (session level, media level, or both): both
/// 
/// whether the attribute value is subject to the charset attribute:
/// not subject to the charset attribute
/// 
/// a one-paragraph explanation of the purpose of the attribute: This
/// attribute defines the mapping from the extension numbers used in
/// packet headers into extension names as documented in
/// specifications and appropriately registered.
#[derive(Debug)]
pub struct ExtMap<'a> {
    pub key: u8, 
    pub value: &'a str,
}

impl<'a> TryFrom<&'a str> for ExtMap<'a> {
    type Error = anyhow::Error;
    /// # Unit Test
    ///
    /// ```
    /// use sdp::attributes::*;
    ///
    /// assert!(ExtMap::try_from("1 urn:ietf:params:rtp-hdrext:toffset").is_ok());
    /// assert!(ExtMap::try_from("2 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time").is_ok());
    /// assert!(ExtMap::try_from("3 urn:3gpp:video-orientation").is_ok());
    /// assert!(ExtMap::try_from("4").is_err());
    /// assert!(ExtMap::try_from("4 name panda").is_err());
    /// ```
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (k, value) = tuple2_from_split(value, ' ', "invalid extmap!")?;
        Ok(Self {
            key: k.parse()?, 
            value, 
        })
    }
}
