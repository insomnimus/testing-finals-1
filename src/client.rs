#[cfg(test)]
mod tests;

use std::{
	fs::File,
	io::{
		self,
		Write,
	},
	path::Path,
};

use snap::write::FrameEncoder;

use crate::{
	encryption::Cipher,
	proto::Header,
	Result,
};

pub fn send_msg<W: Write, C: Cipher>(mut con: W, msg: &str, cipher: &C) -> Result<()> {
	let msg = cipher.encrypt(msg.as_bytes());
	let header = Header::Msg { len: msg.len() };
	bincode::serialize_into(&mut con, &header)?;
	con.write_all(&msg)?;
	Ok(())
}

pub fn send_file<W: Write, C: Cipher>(mut con: W, path: &Path, cipher: &C) -> Result<()> {
	let mut file = File::open(path)?;
	let mut archiver = FrameEncoder::new(Vec::new());
	io::copy(&mut file, &mut archiver)?;
	let mut compressed = archiver.into_inner()?;
	cipher.encrypt_in_place(&mut compressed);

	let header = Header::File {
		name: path
			.file_name()
			.map(|s| s.to_string_lossy().to_string())
			.unwrap_or_else(|| String::from("unnamed_file")),
		len: compressed.len(),
		compressed: true,
	};

	// Send the header.
	bincode::serialize_into(&mut con, &header)?;
	// Send the data.
	con.write_all(&compressed)?;

	Ok(())
}
