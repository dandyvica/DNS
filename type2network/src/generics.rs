//! All functions/trait to convert DNS structures to network order back & forth
use std::io::{Read, Result, Write};
use std::marker::PhantomData;
//use std::cell::Cell;

use crate::{FromNetworkOrder, ToNetworkOrder};

impl<T: ToNetworkOrder> ToNetworkOrder for Option<T> {
    /// ```
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// assert_eq!(Some(0xFF_u8).to_network_order(&mut buffer).unwrap(), 1);
    /// assert_eq!(buffer, &[0xFF]);
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// let r: Option<u8> = None;
    /// assert_eq!(r.to_network_order(&mut buffer).unwrap(), 0);
    /// assert!(buffer.is_empty());
    /// ```    
    fn to_network_order<W: Write>(&self, buffer: &mut W) -> Result<usize> {
        if self.is_none() {
            Ok(0)
        } else {
            self.as_ref().unwrap().to_network_order(buffer)
        }
    }
}

impl<T: FromNetworkOrder> FromNetworkOrder for Option<T> {
    /// ```
    /// use std::io::Cursor;
    /// use type2network::FromNetworkOrder;
    ///
    /// let b = vec![0x12, 0x34, 0x56, 0x78];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut v: Option<u32> = None;
    /// assert!(v.from_network_order(&mut buffer).is_ok());
    /// assert!(v.is_none());
    ///
    /// let b = vec![0x12, 0x34, 0x56, 0x78];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut v: Option<u32> = Some(0u32);
    /// assert!(v.from_network_order(&mut buffer).is_ok());
    /// assert_eq!(v.unwrap(), 0x12345678);
    /// ```
    fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
        if self.is_none() {
            Ok(())
        } else {
            self.as_mut().unwrap().from_network_order(buffer)
        }
    }
}

impl<T: ToNetworkOrder, const N: usize> ToNetworkOrder for [T; N] {
    /// ```
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// assert_eq!([0xFFFF_u16; 10].to_network_order(&mut buffer).unwrap(), 20);
    /// assert_eq!(buffer, &[0xFF; 20]);
    /// ```    
    fn to_network_order<W: Write>(&self, buffer: &mut W) -> Result<usize> {
        let mut length = 0usize;
        let mut buf: Vec<u8> = Vec::new();

        for x in self {
            // first convert x to network bytes
            length += x.to_network_order(&mut buf)?;

            _ = buffer.write(&buf)?;
            buf.clear();
        }

        Ok(length)
    }
}

impl<T: FromNetworkOrder, const N: usize> FromNetworkOrder for [T; N] {
    /// ```
    /// use std::io::Cursor;
    /// use type2network::FromNetworkOrder;
    ///
    /// let b = vec![0x12, 0x34, 0x56, 0x78];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut v = [0u8;4];
    /// assert!(v.from_network_order(&mut buffer).is_ok());
    /// assert_eq!(v, [0x12_u8, 0x34, 0x56, 0x78]);
    ///
    /// let b = vec![0x12, 0x34, 0x56, 0x78];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut v = [0u16;2];
    /// assert!(v.from_network_order(&mut buffer).is_ok());
    /// assert_eq!(v, [0x1234_u16, 0x5678]);
    /// ```
    fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
        for x in self {
            x.from_network_order(buffer)?;
        }
        Ok(())
    }
}

impl<T> ToNetworkOrder for Vec<T>
where
    T: ToNetworkOrder,
{
    /// ```
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// let v = vec![[0xFFFF_u16;3],[0xFFFF;3],[0xFFFF;3]];
    /// assert_eq!(v.to_network_order(&mut buffer).unwrap(), 18);
    /// assert_eq!(&buffer, &[0xFF; 18]);
    /// ```
    fn to_network_order<W: Write>(&self, buffer: &mut W) -> Result<usize> {
        let mut length = 0usize;

        // copy data for each element
        for item in self {
            length += item.to_network_order(buffer)?;
        }

        Ok(length)
    }
}

impl<T> FromNetworkOrder for Vec<T>
where
    T: FromNetworkOrder,
{
    /// ```
    /// use std::io::Cursor;
    /// use type2network::FromNetworkOrder;
    ///
    /// let b = vec![0x12, 0x34, 0x56, 0x78];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut v: Vec<u16> = vec![0_u16;2];
    /// assert!(v.from_network_order(&mut buffer).is_ok());
    /// assert_eq!(v, &[0x1234_u16, 0x5678]);
    /// ```
    fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
        for item in self {
            item.from_network_order(buffer)?;
        }
        Ok(())
    }
}

impl<T> ToNetworkOrder for Box<T>
where
    T: ToNetworkOrder,
{
    /// ```
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// let v = Box::new(vec![[0xFFFF_u16;3],[0xFFFF;3],[0xFFFF;3]]);
    /// assert_eq!(v.to_network_order(&mut buffer).unwrap(), 18);
    /// assert_eq!(&buffer, &[0xFF; 18]);
    /// ```    
    fn to_network_order<W: Write>(&self, buffer: &mut W) -> Result<usize> {
        use std::ops::Deref;
        self.deref().to_network_order(buffer)
    }
}

impl<T> FromNetworkOrder for Box<T>
where
    T: FromNetworkOrder,
{
    /// ```
    /// use std::io::Cursor;
    /// use std::ops::Deref;
    /// use type2network::FromNetworkOrder;
    ///
    /// let b = vec![0x12, 0x34, 0x56, 0x78];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut v = Box::new(vec![0_u16;2]);
    /// assert!(v.from_network_order(&mut buffer).is_ok());
    /// assert_eq!(v.deref(), &[0x1234_u16, 0x5678]);
    /// ```    
    fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
        use std::ops::DerefMut;
        self.deref_mut().from_network_order(buffer)
    }
}

impl<T> ToNetworkOrder for PhantomData<T> {
    fn to_network_order<W: Write>(&self, _buffer: &mut W) -> Result<usize> {
        Ok(0)
    }
}

impl<T> FromNetworkOrder for PhantomData<T> {
    fn from_network_order<R: Read>(&mut self, _buffer: &mut R) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::{from_network_test, to_network_test};

    #[test]
    fn array() {
        // Array of 5 Option<u16>
        to_network_test(
            [None, Some(0x1234_u16), None, Some(0x5678_u16), None],
            4,
            &[0x12_u8, 0x34, 0x56, 0x78],
        );

        from_network_test(
            Some([Some(0_u16); 2]),
            [Some(0x1234_u16), Some(0x5678_u16)],
            &vec![0x12_u8, 0x34, 0x56, 0x78],
        );
    }

    #[test]
    fn vector() {
        // Vec of 5 Option<u16>
        let val = vec![
            [Some(0x1234_u16), Some(0x5678_u16)],
            [Some(0x2345_u16), Some(0x6789_u16)],
            [Some(0x3456_u16), Some(0x789A_u16)],
        ];
        to_network_test(
            val,
            12,
            &[
                0x12_u8, 0x34, 0x56, 0x78, 0x23_u8, 0x45, 0x67, 0x89, 0x34_u8, 0x56, 0x78, 0x9A,
            ],
        );

        from_network_test(
            Some(vec![[Some(0_u16); 2]; 3]),
            vec![
                [Some(0x1234_u16), Some(0x5678_u16)],
                [Some(0x2345_u16), Some(0x6789_u16)],
                [Some(0x3456_u16), Some(0x789A_u16)],
            ],
            &vec![
                0x12_u8, 0x34, 0x56, 0x78, 0x23, 0x45, 0x67, 0x89, 0x34, 0x56, 0x78, 0x9A,
            ],
        );
    }
}
