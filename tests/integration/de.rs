use serde_csv_core::de::{Error, Reader};

#[test]
fn bool_true() {
    let input = b"true";
    let mut reader: Reader<4> = Reader::new();

    let result = reader.deserialize::<bool>(&input[..]);

    assert_eq!(result, Ok((true, 4)))
}

#[test]
fn bool_false() {
    let input = b"false";
    let mut reader: Reader<5> = Reader::new();

    let result = reader.deserialize::<bool>(&input[..]);

    assert_eq!(result, Ok((false, 5)))
}

#[test]
fn bool_empty() {
    let input = b"";
    let mut reader: Reader<0> = Reader::new();

    let result = reader.deserialize::<bool>(&input[..]);

    assert_eq!(result, Err(Error::InvalidBool))
}

#[test]
fn bool_overflow() {
    let input = b"overflow";
    let mut reader: Reader<3> = Reader::new();

    let result = reader.deserialize::<bool>(&input[..]);

    assert_eq!(result, Err(Error::Overflow))
}

#[test]
fn i8_positive() {
    let input = b"123";
    let mut reader: Reader<3> = Reader::new();

    let result = reader.deserialize::<i8>(&input[..]);

    assert_eq!(result, Ok((123, 3)))
}

#[test]
fn i8_negative() {
    let input = b"-123";
    let mut reader: Reader<4> = Reader::new();

    let result = reader.deserialize::<i8>(&input[..]);

    assert_eq!(result, Ok((-123, 4)))
}

#[test]
fn i8_invalid() {
    let input = b"256";
    let mut reader: Reader<3> = Reader::new();

    let result = reader.deserialize::<i8>(&input[..]);

    assert_eq!(result, Err(Error::InvalidInt))
}

#[test]
fn char_valid() {
    let input = b"\xc4\x85";
    let mut reader: Reader<2> = Reader::new();

    let result = reader.deserialize::<char>(&input[..]);

    assert_eq!(result, Ok(('ą', 2)))
}

#[test]
fn char_invalid() {
    let input = b"\xc4\x85\xc4\x85";
    let mut reader: Reader<4> = Reader::new();

    let result = reader.deserialize::<char>(&input[..]);

    assert_eq!(result, Err(Error::InvalidUtf8Char))
}

// Ipv4Addr's implementation of serde::Deserialize calls visit_str
#[test]
fn visit_str() {
    use std::net::Ipv4Addr;

    let input = b"192.168.0.1";
    let mut reader: Reader<11> = Reader::new();

    let result = reader.deserialize::<std::net::Ipv4Addr>(&input[..]);

    assert_eq!(result, Ok((Ipv4Addr::new(192, 168, 0, 1), 11)))
}

#[test]
fn visit_dynamically_allocated_string() {
    let input = b"hello";
    let mut reader: Reader<5> = Reader::new();

    let result = reader.deserialize::<String>(&input[..]);

    assert_eq!(result, Ok((String::from("hello"), 5)))
}

#[test]
fn some() {
    let input = b"123";
    let mut reader: Reader<3> = Reader::new();

    let result = reader.deserialize::<Option<i32>>(&input[..]);

    assert_eq!(result, Ok((Some(123), 3)))
}

#[test]
fn none() {
    let input = b"";
    let mut reader: Reader<0> = Reader::new();

    let result = reader.deserialize::<Option<i32>>(&input[..]);

    assert_eq!(result, Ok((None, 0)))
}

#[test]
fn unit_valid() {
    let input = b"";
    let mut reader: Reader<0> = Reader::new();

    let result = reader.deserialize::<()>(&input[..]);

    assert_eq!(result, Ok(((), 0)))
}

#[test]
fn unit_invalid() {
    let input = b"abcd";
    let mut reader: Reader<4> = Reader::new();

    let result = reader.deserialize::<()>(&input[..]);

    assert_eq!(result, Err(Error::ExpectedEmpty))
}

#[test]
fn empty_records() {
    let input = b",,,";
    let mut reader: Reader<1> = Reader::new();

    let result = reader.deserialize::<(i32, i32)>(&input[..]);

    assert_eq!(result, Err(Error::InvalidInt))
}

#[test]
fn struct_0() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Record;

    let input = b"";
    let mut reader: Reader<0> = Reader::new();

    let result = reader.deserialize::<Record>(&input[..]);

    assert_eq!(result, Ok((Record, 0)))
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

    let result = reader.deserialize::<Record>(&input[..]);

    assert_eq!(result, Ok((Record { x: 0, y: 1 }, 3)))
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

    let result = reader.deserialize::<Record>(&input[..]);

    assert_eq!(result, Ok((Record::B, 1)))
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

    let result = reader.deserialize::<Data>(&input[..]);

    assert_eq!(
        result,
        Ok((
            Data {
                t: (0, 1),
                a: [2, 3, 4, 5],
                s: Struct { x: 6, y: 7 },
            },
            15
        ))
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

    let result = reader.deserialize::<Data>(&input[..]);

    assert_eq!(result, Err(Error::Custom));
}

#[test]
fn array_too_many_fields() {
    let input = b"0,1,2,3";
    let mut reader: Reader<2> = Reader::new();

    let result = reader.deserialize::<[u8; 3]>(&input[..]);

    assert_eq!(result, Ok(([0, 1, 2], 6)));
}

#[test]
#[should_panic(
    expected = "not implemented: `VariantAccess::newtype_variant_seed` is not supported"
)]
fn stateful_enum() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct A {
        x: i32,
        y: i32,
    }

    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct B {
        x: i32,
    }

    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    enum Data {
        A(A),
        B(B),
    }

    let input = b"A,0,1\nB,0";
    let mut reader: Reader<16> = Reader::new();

    let (value, nread) = reader.deserialize::<Data>(&input[..]).unwrap();
    assert_eq!(value, Data::A(A { x: 0, y: 1 }));

    let (value, _) = reader.deserialize::<Data>(&input[nread..]).unwrap();
    assert_eq!(value, Data::B(B { x: 0 }));
}
