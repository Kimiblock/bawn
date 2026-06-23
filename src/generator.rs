use crate::types;

impl crate::types::PortableConfig {
	pub fn new (sandbox_name: &String) -> Self {
		let mut id = String::from("org.kraftland.portable.");
		id.push_str(&sandbox_name.to_string());
		let mut name = String::from("Bawn: ");
		name.push_str(sandbox_name);
		types::PortableConfig {
			Metadata: types::PortableMetadata {
				AppID: id,
				FriendlyName: name,
				StateDirectory: sandbox_name.to_string(),
			}
		}
	}
}
