use super::*;

#[test]
fn test_keyed_xor() {
	let tests = &[
		("password", "input 1"),
		("adsfawefasdf", "bananananananaannanananana"),
		("ünıçöde çhäräçtërş", "çân ënçrypt"),
	];

	for (password, data) in tests {
		let xor = KeyedXor::new(password);
		let encrypted = xor.encrypt(data.as_bytes());
		let decrypted = xor.decrypt(&encrypted);
		assert_eq!(
			data.as_bytes(),
			&decrypted,
			"xor failed to properly encrypt/decrypt"
		);
	}
}

#[test]
fn test_keyed_xor_in_place() {
	let tests = &[
		("password", "input 1"),
		("adsfawefasdf", "bananananananaannanananana"),
		("ünıçöde çhäräçtërş", "çân ënçrypt"),
	];

	for (password, data) in tests {
		let xor = KeyedXor::new(password);
		let mut buf = data.as_bytes().to_vec();
		xor.encrypt_in_place(&mut buf);
		xor.decrypt_in_place(&mut buf);
		assert_eq!(
			data.as_bytes(),
			&buf,
			"xor failed to properly encrypt/decrypt"
		);
	}
}

#[test]
fn test_keyed_xor_cross() {
	let tests = &[
		("password", "input 1"),
		("adsfawefasdf", "bananananananaannanananana"),
		("ünıçöde çhäräçtërş", "çân ënçrypt"),
	];

	for (password, data) in tests {
		let xor = KeyedXor::new(password);
		let mut buf = data.as_bytes().to_vec();
		let encrypted = xor.encrypt(data.as_bytes());
		xor.encrypt_in_place(&mut buf);
		assert_eq!(
			&encrypted, &buf,
			"xor.encrypt and xor.encrypt_in_place yielded different values",
		);
		let decrypted = xor.decrypt(&encrypted);
		xor.decrypt_in_place(&mut buf);

		assert_eq!(
			&decrypted, &buf,
			"xor.decrypt and xor.decrypt_in_place yielded different values",
		);
		assert_eq!(
			data.as_bytes(),
			&decrypted,
			"xor failed to properly encrypt/decrypt"
		);
	}
}
