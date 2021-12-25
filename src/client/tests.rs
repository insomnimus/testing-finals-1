use super::*;
use crate::{
	encryption::KeyedXor,
	Error,
};

#[test]
fn test_send_msg() {
	use std::io::Read;
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
		let expected_header = Header::Msg {
			len: expected_cipher_text.len(),
		};

		// Simulate writing to a socket connection.
		assert_eq!(
			Ok(()), // no error.
			send_msg(&mut dummy_connection, msg, &xor),
		);

		// Now we read.
		let mut dummy_receiver = std::io::Cursor::new(dummy_connection);
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
