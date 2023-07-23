//! All functions/trait to convert DNS structures to network order back & forth
use std::cell::{Cell, OnceCell, RefCell};
use std::io::{Read, Write};

use crate::error::Error;
use crate::{FromNetworkOrder, ToNetworkOrder};

// helper macro for boiler plate definitions
// #[macro_export]
// macro_rules! impl_cell {
//     ($t:ty) => {
//         impl<T> ToNetworkOrder for $t
//         where
//             T: ToNetworkOrder + Copy,
//         {
//             fn to_network_order<W: Write>(&self, buffer: &mut W) -> Result<usize> {
//                 self.get().to_network_order(buffer)
//             }
//         }

//         impl<T> FromNetworkOrder for $t
//         where
//             T: FromNetworkOrder + Default,
//         {
//             fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
//                 let mut v: T = T::default();
//                 v.from_network_order(buffer)?;

//                 self.set(v)?;
//                 Ok(())
//             }
//         }
//     };
// }

impl<T> ToNetworkOrder for Cell<T>
where
    T: ToNetworkOrder + Copy,
{
    /// ```
    /// use std::cell::Cell;
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// let v = Cell::new([[0xFFFF_u16;3],[0xFFFF;3],[0xFFFF;3]]);
    /// assert_eq!(v.to_network_order(&mut buffer).unwrap(), 18);
    /// assert_eq!(&buffer, &[0xFF; 18]);
    /// ```       
    fn to_network_order<W: Write>(&self, buffer: &mut W) -> Result<usize> {
        self.get().to_network_order(buffer)
    }
}

impl<T> FromNetworkOrder for Cell<T>
where
    T: FromNetworkOrder + Default,
{
    /// ```
    /// use std::io::Cursor;
    /// use std::cell::Cell;
    /// use type2network::FromNetworkOrder;
    ///
    /// let b = [0x12, 0x34, 0x56, 0x78];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut v = Cell::new([0_u16;2]);
    /// assert!(v.from_network_order(&mut buffer).is_ok());
    /// assert_eq!(v, Cell::new([0x1234_u16, 0x5678]));
    /// ```      
    fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
        let mut v: T = T::default();
        v.from_network_order(buffer)?;

        self.set(v);
        Ok(())
    }
}

impl<T> ToNetworkOrder for OnceCell<T>
where
    T: ToNetworkOrder + Copy,
{
    /// ```
    /// use std::cell::OnceCell;
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// let mut v = OnceCell::new();
    /// v.set([[0xFFFF_u16;3],[0xFFFF;3],[0xFFFF;3]]).unwrap();
    /// assert_eq!(v.to_network_order(&mut buffer).unwrap(), 18);
    /// assert_eq!(&buffer, &[0xFF; 18]);
    /// ```     
    fn to_network_order<W: Write>(&self, buffer: &mut W) -> Result<usize> {
        match self.get() {
            None => Ok(0),
            Some(v) => v.to_network_order(buffer),
        }
    }
}

impl<T> FromNetworkOrder for OnceCell<T>
where
    T: FromNetworkOrder + Default,
{
    /// ```
    /// use std::io::Cursor;
    /// use std::cell::OnceCell;
    /// use type2network::FromNetworkOrder;
    ///
    /// let b = [0x12, 0x34, 0x56, 0x78];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut v: OnceCell<[u16;2]> = OnceCell::new();
    /// assert!(v.from_network_order(&mut buffer).is_ok());
    /// assert_eq!(v.get().unwrap(), &[0x1234_u16, 0x5678]);
    /// ```      
    fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
        let mut v: T = T::default();
        v.from_network_order(buffer)?;

        match self.set(v) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::Custom(String::from("cell is full"))),
        }
    }
}

impl<T> ToNetworkOrder for RefCell<T>
where
    T: ToNetworkOrder,
{
    /// ```
    /// use std::cell::RefCell;
    /// use type2network::ToNetworkOrder;
    ///
    /// let mut buffer: Vec<u8> = Vec::new();
    /// let v = RefCell::new(vec![[0xFFFF_u16;3],[0xFFFF;3],[0xFFFF;3]]);
    /// assert_eq!(v.to_network_order(&mut buffer).unwrap(), 18);
    /// assert_eq!(&buffer, &[0xFF; 18]);
    /// ```      
    fn to_network_order<W: Write>(&self, buffer: &mut W) -> Result<usize> {
        self.borrow().to_network_order(buffer)
    }
}

impl<T> FromNetworkOrder for RefCell<T>
where
    T: FromNetworkOrder + Default + std::fmt::Debug,
{
    /// ```
    /// use std::io::Cursor;
    /// use std::cell::RefCell;
    /// use type2network::FromNetworkOrder;
    ///
    /// let b = vec![0x12, 0x34, 0x56, 0x78];
    /// let mut buffer = Cursor::new(b.as_slice());
    /// let mut v = RefCell::new([0_u16;2]);
    /// assert!(v.from_network_order(&mut buffer).is_ok());
    /// assert_eq!(v, RefCell::new([0x1234_u16, 0x5678]));
    /// ```     
    fn from_network_order<R: Read>(&mut self, buffer: &mut R) -> Result<()> {
        let mut v = T::default();
        v.from_network_order(buffer)?;

        self.replace(v);
        Ok(())
    }
}
