use serde_csv_core::de::{Error, Reader, Result};

#[test]
fn bool_true() {
    let input = b"true";
    let mut reader: Reader<4> = Reader::new();

    let result: Result<bool> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(true))
}

#[test]
fn bool_false() {
    let input = b"false";
    let mut reader: Reader<5> = Reader::new();

    let result: Result<bool> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(false))
}

#[test]
fn bool_empty() {
    let input = b"";
    let mut reader: Reader<0> = Reader::new();

    let result: Result<bool> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Parse))
}

#[test]
fn bool_overflow() {
    let input = b"overflow";
    let mut reader: Reader<3> = Reader::new();

    let result: Result<bool> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Overflow))
}

#[test]
fn i8_positive() {
    let input = b"123";
    let mut reader: Reader<3> = Reader::new();

    let result: Result<i8> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(123))
}

#[test]
fn i8_negative() {
    let input = b"-123";
    let mut reader: Reader<4> = Reader::new();

    let result: Result<i8> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(-123))
}

#[test]
fn i8_invalid() {
    let input = b"256";
    let mut reader: Reader<3> = Reader::new();

    let result: Result<i8> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Parse))
}

#[test]
fn char_valid() {
    let input = b"\xc4\x85";
    let mut reader: Reader<2> = Reader::new();

    let result: Result<char> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok('ą'))
}

#[test]
fn char_invalid() {
    let input = b"\xc4\x85\xc4\x85";
    let mut reader: Reader<4> = Reader::new();

    let result: Result<char> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Parse))
}

// Ipv4Addr's implementation of serde::Deserialize calls visit_str
#[test]
fn visit_str() {
    use std::net::Ipv4Addr;

    let input = b"192.168.0.1";
    let mut reader: Reader<11> = Reader::new();

    let result: Result<std::net::Ipv4Addr> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(Ipv4Addr::new(192, 168, 0, 1)))
}

#[test]
fn some() {
    let input = b"123";
    let mut reader: Reader<3> = Reader::new();

    let result: Result<Option<i32>> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(Some(123)))
}

#[test]
fn none() {
    let input = b"";
    let mut reader: Reader<0> = Reader::new();

    let result: Result<Option<i32>> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(None))
}

#[test]
fn unit_valid() {
    let input = b"";
    let mut reader: Reader<0> = Reader::new();

    let result: Result<()> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(()))
}

#[test]
fn unit_invalid() {
    let input = b"abcd";
    let mut reader: Reader<4> = Reader::new();

    let result: Result<()> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Parse))
}

#[test]
fn struct_0() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Record;

    let input = b"";
    let mut reader: Reader<0> = Reader::new();

    let result: Result<Record> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(Record))
}

#[test]
fn struct_2() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Record {
        x: i8,
        y: i8,
    }

    let input = b"0,1";
    let mut reader: Reader<2> = Reader::new();

    let result: Result<Record> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(Record { x: 0, y: 1 }))
}

#[test]
fn c_enum() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    enum Record {
        A,
        B,
        C,
    }

    let input = b"B";
    let mut reader: Reader<1> = Reader::new();

    let result: Result<Record> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(Record::B))
}

#[test]
fn compound() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Struct {
        x: i32,
        y: i32,
    }

    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Data {
        t: (i32, i32),
        a: [i32; 4],
        s: Struct,
    }

    let input = b"0,1,2,3,4,5,6,7";
    let mut reader: Reader<2> = Reader::new();

    let result: Result<Data> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(
        result,
        Ok(Data {
            t: (0, 1),
            a: [2, 3, 4, 5],
            s: Struct { x: 6, y: 7 },
        })
    );
}

#[test]
fn compound_missing_fields() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Struct {
        x: i32,
        y: i32,
    }

    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Data {
        t: (i32, i32),
        a: [i32; 4],
        s: Struct,
    }

    let input = b"0,1,2,3\n4,5,6,7\n";
    let mut reader: Reader<2> = Reader::new();

    let result: Result<Data> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Custom));
}

#[test]
fn array_too_many_fields() {
    let input = b"0,1,2,3";
    let mut reader: Reader<2> = Reader::new();

    let result: Result<[u8; 3]> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok([0, 1, 2]));
}
