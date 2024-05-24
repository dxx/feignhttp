use serde::{ser, Serialize};
use serde::ser::Error as SerdeError;
use std::collections::HashMap;
use std::fmt::Display;
use crate::ser::value::StringSerializer;
use crate::error::Error;
use crate::ErrorKind;

impl SerdeError for Error {

    fn custom<T>(msg: T) -> Self where T:Display {
        Error::new(ErrorKind::Serialize(msg.to_string()), None::<Error>)
    }
    
}


pub struct MapSerializer {
    output: HashMap<String, String>
}

pub fn to_map<T>(t: &T) -> Result<HashMap<String, String>, Error>
where
    T: Serialize,
{
    let mut serializer = MapSerializer {
        output: HashMap::new(),
    };
    t.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut MapSerializer {
    type Ok = ();
    type Error = Error;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support bool")), None::<Error>))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support int")), None::<Error>))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support uint")), None::<Error>))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support float")), None::<Error>))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support char")), None::<Error>))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support &str")), None::<Error>))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support &[u8]")), None::<Error>))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support none")), None::<Error>))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::new(ErrorKind::Serialize(String::from("not support some")), None::<Error>))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support uint")), None::<Error>))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support unit_struct")), None::<Error>))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support unit_variant")), None::<Error>))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::new(ErrorKind::Serialize(String::from("not support newtype_struct")), None::<Error>))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::new(ErrorKind::Serialize(String::from("not support newtype_variant")), None::<Error>))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support sequences")), None::<Error>))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support tuple")), None::<Error>))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support tuple_struct")), None::<Error>))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support tuple_variant")), None::<Error>))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::new(ErrorKind::Serialize(String::from("not support struct_variant")), None::<Error>))
    }
}

impl<'a> ser::SerializeMap for &'a mut MapSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
    {
        self.output.insert(
            key.serialize(&mut StringSerializer::new())?,
            value.serialize(&mut StringSerializer::new())?
        );
        Ok(())
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}


impl<'a> ser::SerializeStruct for &'a mut MapSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        self.output.insert(String::from(key), value.serialize(&mut StringSerializer::new())?);
        Ok(())
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
