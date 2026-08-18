//use serde::Serialize;

use serde::{Deserialize, Serialize};

fn default_false()		-> bool {false}

fn default_empty_vec_string()	-> Vec<String> {vec![]}

pub struct CmdOptions {
	pub sandbox_name:		Option<String>,
	pub exec_name:			Option<String>,
	pub action:			Action,
	pub game_mode:			bool,
	pub x11:			bool,
	pub lockdown:			bool,
	pub kvm:			bool,
}

pub enum Action {
	Start,
	Inspect,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
	pub metadata:		Metadata,

	pub exec:		Exec,

	pub system:		SysMgmt,

	#[serde(default)]
	pub network:		Network,

	#[serde(default)]
	pub privacy:		Privacy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysMgmt {
	#[serde(alias = "inhibitSuspend")]
	pub allow_inhibit:	bool,

	#[serde(alias = "inhibitOnBehalf")]
	pub conduct_inhibit:	bool,

	pub uclamp_max:		u32,

	#[serde(alias = "deviceAllow")]
	pub device_allow:	Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
	#[serde(alias = "appID")]
	// Check needed
	pub sandbox_id:		String,
	#[serde(alias = "friendlyName")]
	pub display_name:	String,
	#[serde(alias = "stateDirectory")]
	pub state_directory:	String,

	#[serde(default = "default_config_version")]
	pub config_version:	usize,
}

fn default_config_version () -> usize {0}

#[derive(Debug, Deserialize, Serialize)]
pub struct Exec {
	#[serde(alias = "target")]
	pub target:		String,

	#[serde(alias = "arguments")]
	#[serde(default = "default_empty_vec_string")]
	pub arguments:		Vec<String>,

	#[serde(alias = "overlay")]
	#[serde(default = "default_false")]
	pub overlay:		bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Network {
	#[serde(alias = "enable")]
	pub allow_network:	bool,
	#[serde(alias = "filter")]
	pub enable_filter:	bool,
}

impl Default for Network {
	fn default() -> Self {
		Self {
			allow_network: false,
			enable_filter: false,
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Privacy {
	pub lockdown:		bool,

	#[serde(alias = "x11")]
	pub x11_compat:		bool,

	#[serde(alias = "classicNotifications")]
	pub classic_notif:	bool,

	#[serde(alias = "pipeWire")]
	pub pipewire:		bool,
}

impl Default for Privacy {
	fn default() -> Self {
		Self {
			lockdown: false,
			x11_compat: false,
			classic_notif: false,
			pipewire: false,
		}
	}
}
