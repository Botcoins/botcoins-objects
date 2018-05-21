extern crate bincode;
#[cfg(feature = "serverside")]
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use coin::*;
pub use history::*;
use serde::{Deserialize, Serialize};

pub trait BincodeSerde<'de>: Serialize + Deserialize<'de> {
	fn serialize(&self) -> Vec<u8>;

	fn deserialize(bytes: Vec<u8>) -> Self;
}

#[macro_export]
macro_rules! rocket_response_derive {
	($($id:ident),+) => (
		#[cfg(feature = "serverside")]
		use rocket::{response, Request, Response};
		use BincodeSerde;
		use bincode::{deserialize as bincode_deserialize, serialize as bincode_serialize};

		$(
			#[cfg(feature = "serverside")]
			impl<'r> response::Responder<'r> for $id {
				fn respond_to(self, _: &Request) -> response::Result<'r> {
					use std::io::Cursor;
					use rocket::http::ContentType;

					Response::build()
						.header(ContentType::Plain)
						.sized_body(Cursor::new(self.serialize()))
						.ok()
				}
			}

			impl<'de> BincodeSerde<'de> for $id {
				fn serialize(&self) -> Vec<u8> {
					bincode_serialize(self).unwrap()
				}

				fn deserialize(bytes: Vec<u8>) -> Self {
					bincode_deserialize(&bytes).unwrap()
				}
			}
		)*
	);
}

mod coin;
mod history;
