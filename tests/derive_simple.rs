#[macro_use]
extern crate rkyv;

use bytecheck::CheckBytes;
use ipi::value::{bytes::Bytes, text::Text};
use ipis::{
    class::{metadata::ClassMetadata, Class},
    object::{Object, ToObjectData},
};
use rkyv::{de::deserializers::SharedDeserializeMap, Deserialize};

#[test]
fn test() {
    #[derive(Class, Debug, PartialEq, Archive, Serialize, Deserialize)]
    #[archive(compare(PartialEq))]
    #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
    pub struct MyStruct {
        sub: MySubstruct,
    }

    #[derive(Class, Debug, PartialEq, Archive, Serialize, Deserialize)]
    #[archive(compare(PartialEq))]
    #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
    pub struct MySubstruct {
        unit: (),
        bool: bool,
        i64: i64,
        u64: u64,
        f32: f32,
        f64: f64,
        bytes: Bytes,
        text: Text,
    }

    let value = MyStruct {
        sub: MySubstruct {
            unit: (),
            bool: true,
            i64: 42,
            u64: 42,
            f32: 42.0,
            f64: 42.0,
            bytes: Bytes(vec![0x12, 0x34, 0x56, 0x78]),
            text: Text::with_en_us("hello world!"),
        },
    };

    // Test derived class methods
    assert_eq!(
        MyStruct::class_cursor()
            .sub()
            .unit()
            .__object_name()
            .to_string(),
        "()",
    );
    assert_eq!(
        MyStruct::class_cursor().sub().unit().to_string(),
        "sub.unit",
    );

    // Test derived object methods
    assert_eq!(
        value.cursor().sub().unit().__object_name().to_string(),
        "()",
    );

    // Test derived object values
    assert_eq!(
        ToObjectData::<()>::__get_object_value(
            &value,
            &[Text::with_en_us("sub"), Text::with_en_us("i64")],
        )
        .unwrap()
        .to_string(),
        "42",
    );

    // Test derived object data
    assert_eq!(
        ToObjectData::<()>::__get_object_data(
            &value,
            &[Text::with_en_us("sub"), Text::with_en_us("text")],
        )
        .unwrap()
        .__to_object_value()
        .unwrap()
        .to_string(),
        "hello world!",
    );

    {
        // Serializing
        let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();

        // You can use the safe API for fast zero-copy deserialization
        let archived = rkyv::check_archived_root::<MyStruct>(&bytes[..]).unwrap();
        assert_eq!(archived, &value);
        assert_eq!(archived.sub.i64, 42);
        assert_eq!(&archived.sub.bytes.0, &[0x12, 0x34, 0x56, 0x78]);
        assert_eq!(&archived.sub.text.msg, "hello world!");
        assert_eq!(&archived.sub.text.lang, "en-US");

        // And you can always deserialize back to the original type
        let deserialized: MyStruct = archived
            .deserialize(&mut SharedDeserializeMap::default())
            .unwrap();
        assert_eq!(&deserialized, &value);
    }

    {
        let value = MyStruct::__class_metadata();

        // Serializing
        let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();

        // You can use the safe API for fast zero-copy deserialization
        let archived = rkyv::check_archived_root::<ClassMetadata>(&bytes[..]).unwrap();
        assert_eq!(archived, &value);

        // And you can always deserialize back to the original type
        let deserialized: ClassMetadata = archived
            .deserialize(&mut SharedDeserializeMap::default())
            .unwrap();
        assert_eq!(&deserialized, &value);
    }
}
