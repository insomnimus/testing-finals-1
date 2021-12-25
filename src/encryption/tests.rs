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
