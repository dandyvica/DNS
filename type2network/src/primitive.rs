//! All functions/trait to convert DNS structures to network order back & forth
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result, Write};

use crate::{impl_primitive, FromNetworkOrder, ToNetworkOrder};

// helper macro for boiler plate definitions
#[macro_export]
macro_rules! impl_primitive {
    ($t:ty, $fw:ident, $fr:ident) => {
        impl ToNetworkOrder for $t {
            fn to_network_order<W: Write>(&self, buffer: &mut W) -> Result<usize> {
                buffer.$fw::<BigEndian>(*self as $t)?;
                Ok(std::mem::size_of::<$t>())
            }
        }

        impl FromNetworkOrder for $t {
            fn from_network_order<T: Read>(&mut self, buffer: &mut T) -> Result<()> {
                *self = buffer.$fr::<BigEndian>()?;
                Ok(())
            }
        }
    };
}

// define impl for signed integers
impl ToNetworkOrder for i8 {
    fn to_network_order<V: Write>(&self, buffer: &mut V) -> Result<usize> {
        buffer.write_i8(*self)?;
        Ok(1)
    }
}

impl FromNetworkOrder for i8 {
    fn from_network_order<T: Read>(&mut self, buffer: &mut T) -> Result<()> {
        *self = buffer.read_i8()?;
        Ok(())
    }
}

impl_primitive!(i16, write_i16, read_i16);
impl_primitive!(i32, write_i32, read_i32);
impl_primitive!(i64, write_i64, read_i64);
impl_primitive!(i128, write_i128, read_i128);

// define impl for unsigned integers
impl ToNetworkOrder for u8 {
    fn to_network_order<V: Write>(&self, buffer: &mut V) -> Result<usize> {
        buffer.write_u8(*self)?;
        Ok(1)
    }
}

impl FromNetworkOrder for u8 {
    fn from_network_order<T: Read>(&mut self, buffer: &mut T) -> Result<()> {
        *self = buffer.read_u8()?;
        Ok(())
    }
}

impl_primitive!(u16, write_u16, read_u16);
impl_primitive!(u32, write_u32, read_u32);
impl_primitive!(u64, write_u64, read_u64);
impl_primitive!(u128, write_u128, read_u128);

// floats
impl_primitive!(f32, write_f32, read_f32);
impl_primitive!(f64, write_f64, read_f64);

impl ToNetworkOrder for char {
    /// ```
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer = Vec::new();;
    /// assert_eq!('💯'.to_network_order(&mut buffer).unwrap(), 4);
    /// assert_eq!(buffer, [0, 1, 244, 175]);
    /// ```
    fn to_network_order<V: Write>(&self, buffer: &mut V) -> Result<usize> {
        let u = *self as u32;
        u.to_network_order(buffer)?;
        //println!("u={} buffer={:?}", u, buffer);
        Ok(std::mem::size_of::<char>())
    }
}

impl FromNetworkOrder for char {
    /// ```
    /// use std::io::Cursor;
    /// use type2network::FromNetworkOrder;
    ///
    /// let b = vec![0, 1, 244, 175];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut c = char::default();
    /// assert!(c.from_network_order(&mut buffer).is_ok());
    /// assert_eq!(c, '💯');
    /// ```
    fn from_network_order<T: Read>(&mut self, buffer: &mut T) -> Result<()> {
        // convert first to u32
        let mut u = 0_u32;
        u.from_network_order(buffer)?;
        *self = char::from_u32(u).unwrap();

        Ok(())
    }
}

impl ToNetworkOrder for &[u8] {
    /// ```
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// assert!(&[0x12_u8, 0x34, 0x56, 0x78].to_network_order(&mut buffer).is_ok());
    /// assert_eq!(buffer, &[0x12, 0x34, 0x56, 0x78]);
    /// ```
    fn to_network_order<V: Write>(&self, buffer: &mut V) -> Result<usize> {
        _ = buffer.write(self.as_ref())?;
        Ok(self.len())
    }
}

impl<'a> ToNetworkOrder for &'a str {
    /// ```
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// assert!(&[0x12_u8, 0x34, 0x56, 0x78].to_network_order(&mut buffer).is_ok());
    /// assert_eq!(buffer, &[0x12, 0x34, 0x56, 0x78]);
    /// ```
    fn to_network_order<V: Write>(&self, buffer: &mut V) -> Result<usize> {
        _ = buffer.write(self.as_bytes())?;
        Ok(self.len())
    }
}

impl ToNetworkOrder for String {
    /// ```
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// assert!(String::from("I ❤ 東京").to_network_order(&mut buffer).is_ok());
    /// assert_eq!(buffer, &[73, 32, 226, 157, 164, 32, 230, 157, 177, 228, 186, 172]);
    /// ```    
    fn to_network_order<V: Write>(&self, buffer: &mut V) -> Result<usize> {
        _ = buffer.write(self.as_bytes())?;
        Ok(self.len())
    }
}

// impl<'a> FromNetworkOrder for String {
//     fn from_network_order<'a>(&mut self, buffer: &mut Cursor<&[u8]>) -> Result<()> {
//         // get a reference on [u8]
//         let position = buffer.position() as usize;
//         let inner_data = buffer.get_ref();

//         // first char is the string length
//         let length = inner_data[position] as u8;

//         // move the cursor forward
//         buffer.seek(SeekFrom::Current(length as i64))?;

//         // save data
//         let s = &buffer.get_ref()[position + 1..position + length as usize + 1];
//         let ss = std::str::from_utf8(s)?;
//         self.push_str(ss);

//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use crate::test_helpers::{from_network_test, to_network_test};

    #[test]
    fn to_net() {
        // unsigned ints
        to_network_test(255_u8, 1, &[0xFF]);
        to_network_test(0x1234_u16, 2, &[0x12, 0x34]);
        to_network_test(0x12345678_u32, 4, &[0x12, 0x34, 0x56, 0x78]);
        to_network_test(
            0x1234567812345678_u64,
            8,
            &[0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78],
        );

        // floats
        to_network_test(std::f32::consts::PI, 4, &[0x40, 0x49, 0x0f, 0xdb]);
        to_network_test(
            std::f64::consts::PI,
            8,
            &[0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18],
        );

        // char
        to_network_test('💯', 4, &[0, 1, 244, 175]);
    }

    #[test]
    fn from_net() {
        // unsigned ints
        from_network_test(None, 255_u8, &vec![0xFF]);
        from_network_test(None, 0x1234_u16, &vec![0x12, 0x34]);
        from_network_test(None, 0x12345678_u32, &vec![0x12, 0x34, 0x56, 0x78]);
        from_network_test(
            None,
            0x1234567812345678_u64,
            &vec![0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78],
        );

        // floats
        from_network_test(None, std::f32::consts::PI, &vec![0x40, 0x49, 0x0f, 0xdb]);
        from_network_test(
            None,
            std::f64::consts::PI,
            &vec![0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18],
        );

        // char
        from_network_test(None, '💯', &vec![0, 1, 244, 175]);
    }
}
