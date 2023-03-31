#[derive(serde::Serialize)]
struct Data {
    f: f32,
    s: &'static str,
}

#[test]
fn serialization() {
    let data = Data {
        f: 21.37,
        s: "da,ta",
    };

    let mut buf = [0; 32];
    let nwritten = serde_csv_core::to_slice(&data, &mut buf).unwrap();
    let record = std::str::from_utf8(&buf[..nwritten]).unwrap();

    assert_eq!(record, "21.37,\"da,ta\"\n");
}
