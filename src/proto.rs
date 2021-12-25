use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum HeaderKind {
	Msg,
	File { name: String, compressed: bool },
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Header {
	pub len: usize,
	pub kind: HeaderKind,
}
