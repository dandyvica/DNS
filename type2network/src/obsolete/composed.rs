//Non primitive types
use std::io::{Cursor, Error, ErrorKind, Result};

use crate::{FromNetworkOrder, ToNetworkOrder};

pub struct TLV<T, L, V> {
    tag: T,
    length: L,
    value: V,
}

impl<T, L, V> ToNetworkOrder for TLV<T, L, V>
where
    T: ToNetworkOrder,
    L: ToNetworkOrder,
    V: ToNetworkOrder,
{
    fn to_network_order<V: Write>(&self, buffer: V) -> Result<usize> {
        let mut length = self.tag.to_network_order(buffer)?;
        length += self.length.to_network_order(buffer)?;
        length += self.value.to_network_order(buffer)?;

        Ok(length)
    }
}

impl<T, L, V> FromNetworkOrder for TLV<T, L, V>
where
    T: FromNetworkOrder,
    L: FromNetworkOrder,
    V: FromNetworkOrder,
{
    fn from_network_order<T: Read>(&mut self, buffer: &mut T) -> Result<()> {
        self.tag.from_network_order(buffer)?;
        self.length.from_network_order(buffer)?;
        self.value.from_network_order(buffer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{from_network_test, to_network_test};

    #[test]
    fn tlv() {
        type T1 = TLV<u16, u16, Vec<u8>>;

        let t1 = T1 {
            tag: 0x1234,
            length: 0x5678,
            value: vec![0x72, 0x26, 0x9A, 0x33],
        };

        to_network_test(t1, 8, &[0x12, 0x34, 0x56, 0x78, 0x72, 0x26, 0x9A, 0x33]);

        let buf = vec![0x12, 0x34, 0x56, 0x78, 0x72, 0x26, 0x9A, 0x33];
        let mut buffer = Cursor::new(buf.as_slice());
        let mut v = T1 {
            tag: 0,
            length: 0,
            value: vec![0u8; 4],
        };
        assert!(v.from_network_order(&mut buffer).is_ok());
        assert_eq!(v.tag, 0x1234);
        assert_eq!(v.length, 0x5678);
        assert_eq!(v.value, vec![0x72, 0x26, 0x9A, 0x33]);
    }
}
