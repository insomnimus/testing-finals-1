#[cfg(test)]
mod tests;

pub trait Cipher {
	fn encrypt(&self, plain: &[u8]) -> Vec<u8>;
	fn decrypt(&self, cipher: &[u8]) -> Vec<u8>;
}

pub struct KeyedXor {
	key: String,
}

impl Cipher for KeyedXor {
	fn encrypt(&self, plain: &[u8]) -> Vec<u8> {
		plain
			.iter()
			.zip(self.key.bytes().cycle())
			.map(|(plain_byte, key_byte)| plain_byte ^ key_byte)
			.collect()
	}

	fn decrypt(&self, cipher: &[u8]) -> Vec<u8> {
		// Xor is symmetric.
		self.encrypt(cipher)
	}
}

impl KeyedXor {
	pub fn new<S: AsRef<str>>(key: S) -> Self {
		Self {
			key: key.as_ref().to_string(),
		}
	}
}
