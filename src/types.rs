pub struct CmdOptions {
	pub sandbox_name:		Option<String>,
	pub exec_name:			Option<String>,
	pub action:			Action,
}

pub enum Action {
	Start,
}

#[allow(non_snake_case)]
pub struct PortableConfig {
	pub Metadata:			PortableMetadata,
}

#[allow(non_snake_case)]
pub struct PortableMetadata {
	pub AppID:			String,
	pub FriendlyName:		String,
	pub StateDirectory:		String
}
