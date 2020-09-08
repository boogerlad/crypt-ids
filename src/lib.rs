use aes::Aes128;
use fpe::ff1::{BinaryNumeralString, FF1};
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

#[wasm_bindgen]
pub struct Cryptids {
	foo: FF1::<Aes128>,
}

#[wasm_bindgen]
impl Cryptids {
	#[wasm_bindgen(constructor)]
	pub fn new(bytes: &Uint8Array) -> Cryptids {
		Cryptids {
			foo: FF1::<Aes128>::new(
				&bytes.to_vec(),
				2
			).unwrap()
		}
	}
	pub fn i2s(&self, i: i32) -> String {
		bs58::encode(
			self.foo.encrypt(
				&[],
				&BinaryNumeralString::from_bytes_le(&i.to_le_bytes())
			).unwrap().to_bytes_le()
		).into_string()
	}
	pub fn s2i(&self, s: &str) -> i32 {
		i32::from_le_bytes(
			*std::convert::TryInto::<&[u8; 4]>::try_into(
				&self.foo.decrypt(
					&[],
					&BinaryNumeralString::from_bytes_le(&bs58::decode(s).into_vec().unwrap())
				).unwrap().to_bytes_le()[..]
			).unwrap()
		)
	}
}