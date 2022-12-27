use crate::util::tuple2_from_split;
use std::collections::HashMap;
use std::convert::TryFrom;
use anyhow::Result;

/// This attribute allows parameters that are specific to a
/// particular format to be conveyed in a way that SDP does not
/// have to understand them.  The format must be one of the formats
/// specified for the media.  Format-specific parameters may be any
/// set of parameters required to be conveyed by SDP and given
/// unchanged to the media tool that will use this format.  At most
/// one instance of this attribute is allowed for each format.
/// 
/// It is a media-level attribute, and it is not dependent on
/// charset.
#[derive(Debug)]
pub struct Fmtp<'a> {
    pub key: u8,
    pub values: HashMap<&'a str, Option<&'a str>>
}

impl<'a> TryFrom<&'a str> for Fmtp<'a> {
    type Error = anyhow::Error;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (code, value) = tuple2_from_split(value, ' ', "invalid fmtp!")?;
        let mut values = HashMap::with_capacity(5);
        let key: u8 = code.parse()?;

        for value in value.split(';') {
            let mut value_spt = value.split('=');
            values.insert(value_spt.next().ok_or_else(|| {
                anyhow::anyhow!("invalid fmtp!")
            })?, value_spt.next());
        }

        Ok(Self {
            key,
            values
        })
    }
}
