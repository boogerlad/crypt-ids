use aes::Aes128;
use fpe::ff1::{BinaryNumeralString, FF1};
use wasm_bindgen::prelude::*;
use js_sys::{ArrayBuffer, Uint8Array};

#[wasm_bindgen]
extern {
	pub type Buffer;

	#[wasm_bindgen(method, getter)]
	fn buffer(this: &Buffer) -> ArrayBuffer;

	#[wasm_bindgen(method, getter, js_name = byteOffset)]
	fn byte_offset(this: &Buffer) -> u32;

	#[wasm_bindgen(method, getter)]
	fn length(this: &Buffer) -> u32;
}

#[wasm_bindgen]
pub struct Cryptids {
	foo: FF1::<Aes128>,
}

#[wasm_bindgen]
impl Cryptids {
	#[wasm_bindgen(constructor)]
	pub fn new(buffer: &Buffer) -> Cryptids {
		Cryptids {
			foo: FF1::<Aes128>::new(
				&Uint8Array::new_with_byte_offset_and_length(
					&buffer.buffer(),
					buffer.byte_offset(),
					buffer.length(),
				).to_vec(),
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