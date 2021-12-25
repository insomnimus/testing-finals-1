use std::{
	fs,
	io::{
		self,
		Cursor,
		Read,
		Write,
	},
};

use rand::{
	distributions::Alphanumeric,
	Fill,
	Rng,
};
use snap::{
	read::FrameDecoder,
	write::FrameEncoder,
};
use temp_dir::TempDir;

use super::*;
use crate::{
	encryption::KeyedXor,
	proto::HeaderKind,
	Error,
};

#[test]
fn test_send_msg() {
	let tests = &[
		("password1", "first message; what's up"),
		("password123", "don't use that password"),
		(
			"hp lovecraft",
			"That is not dead which can eternal lie.\nWith strange aeons, even death may die...",
		),
		("dread_pirate_boomers", "please give me an A"),
	];

	for (key, msg) in tests {
		let mut dummy_connection = Vec::new();
		let xor = KeyedXor::new(key);
		let expected_cipher_text = xor.encrypt(msg.as_bytes());
		let expected_header = Header {
			kind: HeaderKind::Msg,
			len: expected_cipher_text.len(),
		};

		// Simulate writing to a socket connection.
		assert_eq!(
			Ok(()), // no error.
			send_msg(&mut dummy_connection, msg, &xor),
		);

		// Now we read.
		let mut dummy_receiver = Cursor::new(dummy_connection);
		// Read the header from the "connection".

		assert_eq!(
			Ok(expected_header),
			bincode::deserialize_from(&mut dummy_receiver).map_err(Error::from),
		);

		// Assert that rest of the data is the encrypted text.
		let mut received_cipher_text = Vec::new();
		dummy_receiver
			.read_to_end(&mut received_cipher_text)
			.unwrap();
		assert_eq!(
			&expected_cipher_text, &received_cipher_text,
			"sent and received ciphers are different"
		);

		// Now we decrypt it with our key and assert that it's the message we sent.
		let decrypted_msg = xor.decrypt(&received_cipher_text);
		assert_eq!(msg.as_bytes(), &decrypted_msg, "decryption failed",);
	}
}

#[test]
fn test_send_file() {
	let tmp = TempDir::new().unwrap();

	let tests = &[
		("demo.txt", "password4321"),
		("with space.jpeg", "some random password"),
		("qwertyuıopğ.bin", "asdfasdf"),
		("notavirus.cmd", "itwasavirus"),
		("int main", "(**char argv)"),
	];

	for (file_name, password) in tests {
		let mut random_data = [0_u8; 2048];
		random_data
			.try_fill(&mut rand::thread_rng())
			.expect("failed to fill buffer with random bytes");
		let path = tmp.child(file_name);
		// Write the data to the file.
		fs::write(&path, &random_data).expect("failed to generate test file");
		let mut dummy_con = Vec::new();
		let xor = KeyedXor::new(password);

		// "Send" the file.
		assert_eq!(Ok(()), send_file(&mut dummy_con, &path, &xor));

		// Read past the header, assert that it's a file.
		let mut dummy_receiver = Cursor::new(dummy_con);
		let header: Header = bincode::deserialize_from(&mut dummy_receiver)
			.expect("received data does not start with a valid header");
		let len = header.len;
		match header.kind {
			HeaderKind::File { name, compressed } if name.eq(file_name) && compressed => (),
			x => panic!("header unexpected header: {:?}", x),
		};
		// Now read the compressed and encrypted  file.
		let mut received_data = Vec::new();
		dummy_receiver.read_to_end(&mut received_data).unwrap();

		assert_eq!(
			len,
			received_data.len(),
			"thea header's len property is incorrect"
		);

		xor.decrypt_in_place(&mut received_data);
		let mut uncompressed = Vec::new();
		let mut unarchiver = FrameDecoder::new(Cursor::new(received_data));
		io::copy(&mut unarchiver, &mut uncompressed).expect("failed to decompress received file");

		assert_eq!(
			&random_data[..],
			&uncompressed[..],
			"received data is not the same as sent data",
		);
	}
}
