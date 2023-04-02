#[test]
fn serialize_unit() {
    let data = ();

    let mut writer = csv_core::Writer::new();
    let mut buf = [0; 32];
    let nwritten = serde_csv_core::to_slice(&mut writer, &data, &mut buf).unwrap();
    let record = std::str::from_utf8(&buf[..nwritten]).unwrap();

    assert_eq!(record, "\"\"\n");
}

#[test]
fn serialize_pair_units() {
    let data = ((), ());

    let mut writer = csv_core::Writer::new();
    let mut buf = [0; 32];
    let nwritten = serde_csv_core::to_slice(&mut writer, &data, &mut buf).unwrap();
    let record = std::str::from_utf8(&buf[..nwritten]).unwrap();

    assert_eq!(record, ",\n");
}

#[test]
fn serialize_none() {
    let data: Option<()> = None;

    let mut writer = csv_core::Writer::new();
    let mut buf = [0; 32];
    let nwritten = serde_csv_core::to_slice(&mut writer, &data, &mut buf).unwrap();
    let record = std::str::from_utf8(&buf[..nwritten]).unwrap();

    assert_eq!(record, "\"\"\n");
}

#[test]
fn serialize_some() {
    let data: Option<()> = None;

    let mut writer = csv_core::Writer::new();
    let mut buf = [0; 32];
    let nwritten = serde_csv_core::to_slice(&mut writer, &data, &mut buf).unwrap();
    let record = std::str::from_utf8(&buf[..nwritten]).unwrap();

    assert_eq!(record, "\"\"\n");
}

#[test]
fn serialize_empty_slice() {
    let data: &[i32] = &[];

    let mut writer = csv_core::Writer::new();
    let mut buf = [0; 32];
    let nwritten = serde_csv_core::to_slice(&mut writer, &data, &mut buf).unwrap();
    let record = std::str::from_utf8(&buf[..nwritten]).unwrap();

    assert_eq!(record, "\"\"\n");
}

#[test]
fn serialize_slice() {
    let data: &[i32] = &[0, 1, 2, 3];

    let mut writer = csv_core::Writer::new();
    let mut buf = [0; 32];
    let nwritten = serde_csv_core::to_slice(&mut writer, &data, &mut buf).unwrap();
    let record = std::str::from_utf8(&buf[..nwritten]).unwrap();

    assert_eq!(record, "0,1,2,3\n");
}

#[test]
fn serialize_string_with_comma() {
    let data = "a,b,c";

    let mut writer = csv_core::Writer::new();
    let mut buf = [0; 32];
    let nwritten = serde_csv_core::to_slice(&mut writer, &data, &mut buf).unwrap();
    let record = std::str::from_utf8(&buf[..nwritten]).unwrap();

    assert_eq!(record, "\"a,b,c\"\n");
}

#[test]
fn serialize_compound() {
    #[derive(serde::Serialize)]
    struct Struct {
        x: i32,
        y: i32
    }

    #[derive(serde::Serialize)]
    struct Data {
        t: (i32, i32),
        a: [i32; 4],
        s: Struct
    }
    
    let data = Data {
        t: (0, 1),
        a: [2, 3, 4, 5],
        s: Struct { x: 6, y: 7 }
    };

    let mut writer = csv_core::Writer::new();
    let mut buf = [0; 32];
    let nwritten = serde_csv_core::to_slice(&mut writer, &data, &mut buf).unwrap();
    let record = std::str::from_utf8(&buf[..nwritten]).unwrap();

    assert_eq!(record, "0,1,2,3,4,5,6,7\n");
}
