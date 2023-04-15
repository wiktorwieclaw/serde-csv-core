use serde_csv_core::de::{Error, Reader, Result};

#[test]
fn bool_true() {
    let input = b"true";
    let mut reader = Reader::new([0; 4]);

    let result: Result<bool> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(true))
}

#[test]
fn bool_false() {
    let input = b"false";
    let mut reader = Reader::new([0; 5]);

    let result: Result<bool> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(false))
}

#[test]
fn bool_empty() {
    let input = b"";
    let mut reader = Reader::new([0; 0]);

    let result: Result<bool> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Parse))
}

#[test]
fn bool_overflow() {
    let input = b"overflow";
    let mut reader = Reader::new([0; 3]);

    let result: Result<bool> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Overflow))
}

#[test]
fn i8_positive() {
    let input = b"123";
    let mut reader = Reader::new([0; 3]);

    let result: Result<i8> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(123))
}

#[test]
fn i8_negative() {
    let input = b"-123";
    let mut reader = Reader::new([0; 4]);

    let result: Result<i8> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(-123))
}

#[test]
fn i8_invalid() {
    let input = b"256";
    let mut reader = Reader::new([0; 3]);

    let result: Result<i8> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Parse))
}

#[test]
fn unit_valid() {
    let input = b"";
    let mut reader = Reader::new([0; 0]);

    let result: Result<()> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(()))
}

#[test]
fn unit_invalid() {
    let input = b"abcd";
    let mut reader = Reader::new([0; 0]);

    let result: Result<()> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Err(Error::Parse))
}

#[test]
fn struct_0() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Record;

    let input = b"";
    let mut reader = Reader::new([0; 0]);

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
    let mut reader = Reader::new([0; 3]);

    let result: Result<Record> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(result, Ok(Record { x: 0, y: 1 }))
}

#[test]
fn compound() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Struct {
        x: i8,
        y: i8,
    }

    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Data {
        t: Struct,
        a: Struct,
        s: Struct,
    }

    let input = b"0,1,2,3,6,7\n";
    let mut reader = Reader::new([0; 14]);

    let result: Result<Data> = reader.deserialize_from_slice(&input[..]);

    assert_eq!(
        result,
        Ok(Data {
            t: Struct { x: 0, y: 1 },
            a: Struct { x: 2, y: 3 },
            s: Struct { x: 6, y: 7 },
        })
    );
}
