use super::util::tuple3_from_split;
use std::net::IpAddr;
use anyhow::ensure;
use super::{
    NetKind,
    AddrKind
};

use std::{
    convert::TryFrom,
    fmt
};

#[derive(Debug)]
pub struct Addr {
    pub ip: IpAddr,
    /// IPv6 multicast does not use TTL scoping, and hence the TTL value MUST
    /// NOT be present for IPv6 multicast.  It is expected that IPv6 scoped
    /// addresses will be used to limit the scope of conferences.
    pub ttl: Option<u16>,
    pub count: Option<u8>
}

/// Connection Information
///
/// The "c=" line (connection-field) contains information necessary to
/// establish a network connection.
#[derive(Debug)]
pub struct Connection {
    /// <nettype>  is a text string giving the type of network.  Initially,
    /// "IN" is defined to have the meaning "Internet".
    pub nettype: NetKind,
    /// <addrtype>  is a text string giving the type of the address that
    /// follows.  Initially, "IP4" and "IP6" are defined.
    pub addrtype: AddrKind,
    /// (<connection-address>) is the connection address.
    /// Additional subfields MAY be added after the connection address
    /// depending on the value of the <addrtype> subfield.
    pub connection_address: Addr,
}

impl fmt::Display for Connection {
    /// # Unit Test
    ///
    /// ```
    /// use sdp::*;
    /// use sdp::connection::*;
    ///
    /// let temp = "IN IP4 0.0.0.0".to_string();
    /// let connection = Connection {
    ///     nettype: NetKind::IN,
    ///     addrtype: AddrKind::IP4,
    ///     connection_address: Addr {
    ///         ip: "0.0.0.0".parse().unwrap(),
    ///         ttl: None,
    ///         count: None
    ///     }
    /// };
    ///
    /// assert_eq!(format!("{}", connection), temp);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{} {} {}", 
            self.nettype, 
            self.addrtype, 
            self.connection_address
        )
    }
}

impl<'a> TryFrom<&'a str> for Connection {
    type Error = anyhow::Error;
    /// # Unit Test
    ///
    /// ```
    /// use sdp::*;
    /// use sdp::connection::*;
    /// use std::convert::*;
    /// use std::net::IpAddr;
    ///
    /// let temp = "IN IP4 0.0.0.0";
    /// let addr: IpAddr = "0.0.0.0".parse().unwrap();
    /// let instance: Connection = Connection::try_from(temp).unwrap();
    /// 
    /// assert_eq!(instance.nettype, NetKind::IN);
    /// assert_eq!(instance.addrtype, AddrKind::IP4);
    /// assert_eq!(instance.connection_address.ip, addr);
    /// assert_eq!(instance.connection_address.ttl, None);
    /// assert_eq!(instance.connection_address.count, None);
    /// ```
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (n, a, c) = tuple3_from_split(value, ' ', "invalid connection information!")?;
        Ok(Self {
            nettype: NetKind::try_from(n)?,
            addrtype: AddrKind::try_from(a)?,
            connection_address: Addr::try_from(c)?,
        })
    }
}

impl fmt::Display for Addr {
    /// # Unit Test
    ///
    /// ```
    /// use sdp::*;
    /// use sdp::connection::*;
    ///
    /// let temp = "0.0.0.0/127/2".to_string();
    /// let connection = Addr {
    ///     ttl: Some(127),
    ///     count: Some(2),
    ///     ip: "0.0.0.0".parse().unwrap()
    /// };
    ///
    /// assert_eq!(format!("{}", connection), temp);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.ip)?;
        
        if let Some(ttl) = self.ttl {
            write!(f, "/{}", ttl)?;
        }
        
        if let Some(count) = self.count {
            write!(f, "/{}", count)?;
        }
        
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for Addr {
    type Error = anyhow::Error;
    /// # Unit Test
    ///
    /// ```
    /// use sdp::*;
    /// use sdp::connection::*;
    /// use std::convert::*;
    /// use std::net::IpAddr;
    ///
    /// let temp = "0.0.0.0/127/2";
    /// let addr: IpAddr = "0.0.0.0".parse().unwrap();
    /// let instance: Addr = Addr::try_from(temp).unwrap();
    /// 
    /// assert_eq!(instance.ip, addr);
    /// assert_eq!(instance.ttl, Some(127));
    /// assert_eq!(instance.count, Some(2));
    /// ```
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let values = value.split('/').collect::<Vec<&str>>();
        ensure!(!values.is_empty(), "invalid connection information!");
        Ok(Self {
            ip: values[0].parse()?,
            ttl: if let Some(t) = values.get(1) { Some(t.parse()?) } else { None },
            count: if let Some(c) =  values.get(2) { Some(c.parse()?) } else { None}
        })
    }
}
