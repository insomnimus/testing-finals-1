use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum PacketHeader {
	Chat {
		len: u32,
	},
	File {
		name: String,
		len: u64,
		compressed: bool,
	},
}
