#[cfg(test)]
mod tests;

use std::{
	fs::File,
	io::{
		self,
		Read,
		Write,
	},
	path::Path,
};

use snap::write::FrameEncoder;

use crate::{
	encryption::Cipher,
	proto::{
		Header,
		HeaderKind,
	},
	Result,
};

pub fn send_msg<W: Write, C: Cipher>(mut con: W, msg: &str, cipher: &C) -> Result<()> {
	let msg = cipher.encrypt(msg.as_bytes());
	let header = Header {
		kind: HeaderKind::Msg,
		len: msg.len(),
	};
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

	let header = Header {
		kind: HeaderKind::File {
			name: path
				.file_name()
				.map(|s| s.to_string_lossy().to_string())
				.unwrap_or_else(|| String::from("unnamed_file")),
			compressed: true,
		},
		len: compressed.len(),
	};

	// Send the header.
	bincode::serialize_into(&mut con, &header)?;
	// Send the data.
	con.write_all(&compressed)?;

	Ok(())
}

pub fn read_header<R: Read>(mut con: R) -> Result<Header> {
	bincode::deserialize_from(&mut con).map_err(|e| e.into())
}
