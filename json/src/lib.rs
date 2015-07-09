//Sample explaining how use rustc-serialize crate

extern crate chrono;
extern crate rustc_serialize;

use std::collections::BTreeMap;

use rustc_serialize::json::{self, Json, ToJson, DecodeResult};
use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};

use chrono::*;

// Using wrapper object to support serialization to 3rd party Structs
#[derive(Debug)]
pub struct MyDate {
	date: DateTime<UTC>
}

impl MyDate {
	fn now() -> MyDate {
		MyDate{ date: UTC::now()}
	}
}


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Accomplishment {
  name: String,
  accomplishment_type: String,
  date: MyDate,
  roles: Vec<String>
}


impl Accomplishment {
  pub fn new(name: &str, acc_type: &str, date: MyDate, roles: Vec<String>) -> Accomplishment {
    Accomplishment{name: name.to_string(), 
      accomplishment_type: acc_type.to_string(),
      date: date, 
      roles: roles }
  }
}

//Chrono library alreay has support for serilization as a feature. 
//This struct uses that feature
#[derive(Debug, RustcDecodable, RustcEncodable)]
struct User {
	birth_day: DateTime<UTC>
}

trait JsonEncode {
  fn encode(&self) -> String;
}


trait JsonDecode  {
	fn decode<T: rustc_serialize::Decodable>(&self) -> DecodeResult<T>;
}

// JSON decode for string 
impl <'a> JsonDecode for &'a str {
	fn decode< T: rustc_serialize::Decodable>(&self) -> DecodeResult<T> {
		json::decode(self)
	}
}

// Implement Encodable to MyDate
impl Encodable for MyDate {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let j_str: String = self.date.to_string().to_owned();
        let j_str_slice: &str = &j_str[..];
        s.emit_str(j_str_slice)
    }
}

// Implement Decodable to MyDate
impl Decodable for MyDate {
    fn decode<D: Decoder>(d: &mut D) -> Result<MyDate, D::Error> {
        Ok(MyDate::now()) //TODO convert string to date
    }
}

#[test]
fn test_Accomplishment_encode() {
    let a = Accomplishment {
        name: "name".to_string(),
        accomplishment_type: "accType".to_string(),
        date: MyDate::now(),
        roles: Vec::new()
    };

    println!("{:?}", a);

    let b = Accomplishment::new("sandarenu", "winner", MyDate::now(), Vec::new());

	let encoded = json::encode(&a);
	println!("{:?}", encoded);
}

#[test]
fn test_user_encode() {
	let a = User {birth_day: UTC::now()};
	let encoded = json::encode(&a);
	println!("{:?}", encoded);
}
