use std::io::{Error, ErrorKind, Result, Read};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use derive_enum::{FromStr, TryFrom};
use type2network::{FromNetworkOrder, ToNetworkOrder};
use type2network_derive::{FromNetwork, ToNetwork};

// RR Class values: https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4
/// ```
/// use std::convert::TryFrom;
/// use std::str::FromStr;
/// use dnslib::rfc1035::qclass::QClass;
///
/// assert_eq!(QClass::from_str("ANY").unwrap(), QClass::ANY);
/// assert!(QClass::from_str("FOO").is_err());
/// assert_eq!(QClass::try_from(2).unwrap(), QClass::CS);
/// assert!(QClass::try_from(110).is_err());
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, FromStr, TryFrom, ToNetwork, FromNetwork)]
#[repr(u16)]
pub enum QClass {
    #[default]
    IN = 1, // the Internet
    CS = 2, // the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CH = 3, // the CHAOS class
    HS = 4, // Hesiod [Dyer 87]
    ANY = 255,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_network_test, to_network_test};

    #[test]
    fn network() {
        let q = QClass::ANY;
        to_network_test(&q, 2, &[0x00, 0xFF]);
        from_network_test(None, &q, &vec![0x00, 0xFF]);
    }
}