//use serde::Serialize;

pub struct CmdOptions {
	pub sandbox_name:		Option<String>,
	pub exec_name:			Option<String>,
	pub action:			Action,
}

pub enum Action {
	Start,
	Inspect,
}

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
pub struct PortableConfig {
	pub metadata:			PortableMetadata,
	pub exec:			PortableExec,
}

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
pub struct PortableMetadata {
	pub appID:			String,
	pub friendlyName:		String,
	pub stateDirectory:		String
}

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
pub struct PortableExec {
	pub target:			String,
	pub arguments:			Vec<String>,
}
