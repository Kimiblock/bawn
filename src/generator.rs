use crate::types;
use toml;

impl crate::types::PortableConfig {
	pub fn new (sandbox_name: &String) -> Self {
		let mut id = String::from("org.kraftland.portable.");
		id.push_str(&sandbox_name.to_string());
		let mut name = String::from("Bawn-transient-");
		name.push_str(sandbox_name);
		types::PortableConfig {
			metadata: types::PortableMetadata {
				appID: id,
				friendlyName: name,
				stateDirectory: sandbox_name.to_string(),
			},
			exec: types::PortableExec {
				target: "bash".to_string(),
				arguments: vec![
					String::from("--noprofile"),
					String::from("--rcfile"),
					String::from("/run/bashrc"),
					String::from("-i"),
				],
			},
			system: types::PortableSystemOpts {
				deviceAllow: vec![],
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
