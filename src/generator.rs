use crate::types;
use toml;

impl crate::types::PortableConfig {
	pub fn new (sandbox_name: &String) -> Self {
		let mut id = String::from("org.kraftland.portable.");
		id.push_str(&sandbox_name.to_string());
		let mut name = String::from("Bawn of Portable: ");
		name.push_str(sandbox_name);
		types::PortableConfig {
			metadata: types::PortableMetadata {
				appID: id,
				friendlyName: name,
				stateDirectory: sandbox_name.to_string(),
			}
		}
	}
	pub fn print (self: &Self) {
		let result = toml::to_string_pretty(&self);
		let content = match result {
			Err(error) => {
				panic!("Unable to produce configuration: {}", error);
			}
			Ok(content) => {content}
		};
		println!("{content}")
	}
}
