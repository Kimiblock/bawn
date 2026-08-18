use crate::types;
use toml;

impl crate::types::Config {
	pub fn new (sandbox_name: &String) -> Self {
		let mut id = String::from("org.kraftland.portable.");
		id.push_str(&sandbox_name.to_string());
		let mut name = String::from("Bawn-transient-");
		name.push_str(sandbox_name);
		let mut state_dir = String::from(sandbox_name);
		state_dir.push_str("_Data");
		types::Config {
			metadata: types::Metadata {
				sandbox_id:		id,
				display_name:		name,
				state_directory:	state_dir,
				config_version:		20,
			},
			network: types::Network {
				allow_network:		true,
				enable_filter:		false,
			},
			exec: types::Exec {
				overlay:	false,
				target:		"bash".to_string(),
				arguments: vec![
					String::from("-i"),
				],
			},
			system: types::SysMgmt {
				allow_inhibit:		false,
				conduct_inhibit:	false,
				uclamp_max:		0,
				device_allow:		vec![],
			},
			privacy: types::Privacy {
				lockdown:	true,
				x11_compat:	false,
				classic_notif:	false,
				pipewire:	false,
			}
		}
	}
	pub fn to_string (self: &Self) -> Result<String, toml::ser::Error> {
		toml::to_string_pretty(&self)
	}
	pub fn print (self: &Self) {
		let result = self.to_string();
		let content = match result {
			Err(error) => {
				panic!("Unable to produce configuration: {}", error);
			}
			Ok(content) => {content}
		};
		println!("{content}")
	}
}
