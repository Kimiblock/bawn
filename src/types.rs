//use serde::Serialize;

pub struct CmdOptions {
	pub sandbox_name:		Option<String>,
	pub exec_name:			Option<String>,
	pub action:			Action,
	pub game_mode:			bool,
	pub x11:			bool,
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
	pub system:			PortableSystemOpts,
	pub privacy:			PortablePrivacyOpts,
}

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
pub struct PortablePrivacyOpts {
	pub lockdown:			bool,
	pub x11:			bool,
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

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
pub struct PortableSystemOpts {
	pub deviceAllow:		Vec<String>,
}
