use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum Header {
	Msg {
		len: usize,
	},
	File {
		name: String,
		len: usize,
		compressed: bool,
	},
}
