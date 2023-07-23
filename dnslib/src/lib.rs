pub mod error;
// pub mod macros;
pub mod rfc1035;
//pub mod rfc4034;
//pub mod rfc6891;
// pub mod util;

#[cfg(test)]
use type2network::{FromNetworkOrder, ToNetworkOrder};
use type2network_derive::{FromNetwork, ToNetwork};


// used for boiler plate unit tests for integers, floats etc
#[cfg(test)]
pub fn to_network_test<T: ToNetworkOrder>(val: &T, size: usize, v: &[u8]) {
    let mut buffer: Vec<u8> = Vec::new();
    assert_eq!(val.to_network_order(&mut buffer).unwrap(), size);
    assert_eq!(buffer, v);
}

#[cfg(test)]
pub fn from_network_test<'a, T>(def: Option<T>, val: &T, buf: &'a Vec<u8>)
where
    T: FromNetworkOrder + Default + std::fmt::Debug + std::cmp::PartialEq,
{
    let mut buffer = std::io::Cursor::new(buf.as_slice());
    let mut v: T = if def.is_none() {
        T::default()
    } else {
        def.unwrap()
    };
    assert!(v.from_network_order(&mut buffer).is_ok());
    assert_eq!(&v, val);
}
