use feignhttp::{ser, ErrorKind};
use serde::Serialize;

#[derive(Serialize)]
pub struct Header {
    pub id: u32,
    pub token: String,
    pub accept: &'static str,
}

#[test]
fn test_struct_to_map() {
    let header = Header {
        id: 10000,
        token: String::from("MDEwOlJlcG9zaXRvcnkzNDUyNTk4OTA="),
        accept: "application/json",
    };
    let map = ser::to_map(&header).unwrap();
    println!("{:?}", map);
}

#[derive(Serialize)]
pub struct IdList {
    pub ids: Vec<u32>,
}

#[test]
fn test_ser_error() {
    let id_list = IdList {
        ids: vec![100, 200, 300]
    };
    let r = ser::to_map(&id_list);
    match r {
        Err(e) => {
            if let ErrorKind::Serialize(msg) = e.error_kind() {
                assert_eq!("not support sequences", msg);
            }
        },
        Ok(_) => {
            panic!();
        }
    }
}
