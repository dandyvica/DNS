use std::io::{Error, ErrorKind, Result, Read};

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

// some tests for structs
use type2network::{FromNetworkOrder, ToNetworkOrder};
use type2network_derive::{FromNetwork, ToNetwork};
use derive_enum::TryFrom;

// used for boiler plate unit tests for integers, floats etc
pub fn to_network_test<T: ToNetworkOrder>(val: &T, size: usize, v: &[u8]) {
    let mut buffer: Vec<u8> = Vec::new();
    assert_eq!(val.to_network_order(&mut buffer).unwrap(), size);
    assert_eq!(buffer, v);
}

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

#[test]
#[allow(dead_code)]
fn struct_unit() {
    #[derive(ToNetwork, FromNetwork)]
    struct Unit;
}

#[test]
fn struct_basic() {
    #[derive(Debug, Default, PartialEq, ToNetwork, FromNetwork)]
    struct Point {
        x: u16,
        y: u16,
    }

    let pt = Point {
        x: 0x1234,
        y: 0x5678,
    };
    to_network_test(&pt, 4, &[0x12, 0x34, 0x56, 0x78]);
    from_network_test(None, &pt, &vec![0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn struct_tuple_basic() {
    #[derive(Debug, Default, PartialEq, ToNetwork, FromNetwork)]
    struct Point(u16, u16);

    let pt = Point(0x1234, 0x5678);
    to_network_test(&pt, 4, &[0x12, 0x34, 0x56, 0x78]);
    from_network_test(None, &pt, &vec![0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn struct_one_typeparam() {
    #[derive(Debug, Default, PartialEq, ToNetwork, FromNetwork)]
    struct Point<T>
    where
        T: ToNetworkOrder + FromNetworkOrder,
    {
        x: T,
        y: T,
    }

    let pt = Point::<u16> {
        x: 0x1234,
        y: 0x5678,
    };
    to_network_test(&pt, 4, &[0x12, 0x34, 0x56, 0x78]);
    from_network_test(None, &pt, &vec![0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn struct_lifetime() {
    #[derive(Debug, PartialEq, ToNetwork)]
    struct Data<'a, T, V>
    where
        T: ToNetworkOrder,
        V: ToNetworkOrder,
    {
        x: &'a str,
        y: T,
        z: Option<V>,
    }

    let pt = Data::<Option<u16>, Vec<Option<u8>>> {
        x: &"\x01\x02\x03\x04\x05\x06\x07\x08",
        y: Some(0x090A),
        z: Some(vec![None, Some(0x0B), Some(0x0C)]),
    };
    to_network_test(
        &pt,
        12,
        &[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
        ],
    );
}

// #[test]
// #[allow(dead_code)]
// fn enum_simple() {
//     #[derive(ToNetwork)]
//     enum Color {
//         White,
//         Black,
//     }

//     let b = Color::White;
//     to_network_test(&b, 1, &[0x00]);
// }

// #[test]
// #[allow(dead_code)]
// fn enum_message() {
//     #[derive(ToNetwork)]
//     enum Message {
//         Move { x: u16, y: u16 },
//         Write(String),
//         ChangeColor(u16, u16, u16),
//     }

//     let m = Message::Move {
//         x: 0x1234,
//         y: 0x5678,
//     };
//     to_network_test(&m, 4, &[0x12, 0x34, 0x56, 0x78]);

//     let m = Message::ChangeColor(0x1234, 0x5678, 0x9ABC);
//     to_network_test(&m, 6, &[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]);
// }

#[test]
#[allow(dead_code)]
fn enum_cstyle() {
    #[derive(Copy, Clone, TryFrom, ToNetwork, FromNetwork)]
    #[repr(u16)]
    enum Choice {
        A = 1,
        B = 3,
        C = 5,
    }
}
