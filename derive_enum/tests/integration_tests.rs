use derive_enum::{FromStr, TryFrom};
use std::str::FromStr;

#[test]
#[allow(dead_code)]
fn enum_simple() {
    #[derive(Debug, PartialEq, FromStr)]
    enum Color {
        White,
        Black,
    }

    let b = Color::from_str("White").unwrap();
    assert_eq!(b, Color::White);

    #[derive(Debug, PartialEq, FromStr, TryFrom)]
    enum Choice {
        Yes = 0,
        No = 1,
    }

    let c = Choice::from_str("Yes").unwrap();
    assert_eq!(c, Choice::Yes);

    let c = Choice::try_from(1).unwrap();
    assert_eq!(c, Choice::No);

    #[derive(Debug, PartialEq, FromStr)]
    enum Answer {
        A = 1000,
        B,
        C,
    }

    let a = Answer::from_str("C").unwrap();
    assert_eq!(a, Answer::C);

    #[derive(Debug, PartialEq, FromStr)]
    enum Message {
        Move { x: u16, y: u16 },
        Write(String),
        ChangeColor(u16, u16, u16),
    }
}
