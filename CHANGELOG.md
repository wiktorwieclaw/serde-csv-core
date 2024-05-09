# 0.3
- Implemented `Debug` trait for `Writer`, `Reader`, `Serializer`, `Deserializer`
- Implemented `Default` trait for `Writer`, `Reader`
- Replaced `Writer::from_inner` with `Writer::from_builder`
- Replaced `Reader::from_inner` with `Reader::from_builder`
- Removed `Writer::into_inner` and `Reader::into_inner`
- Renamed `Writer::serialize_to_slice` to `Writer::serialize`
- Renamed `Reader::deserialize_from_slice` to `Reader::deserialize`
- Removed ability to serialize and deserialize newtype enum variants. This could lead to situations
  where serializer would produce variable length records, if two variants held structs with
  different number of fields. It was decided that this behavior is bugprone. 