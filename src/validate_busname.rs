use crate::types::CmdOptions;

impl CmdOptions {
	pub fn is_valid(self: &Self) -> bool {
		let (valid, err) = validate_bus_name(&self.sandbox_name);
		if ! valid {
			println!("Sandbox name is invalid: {}", err);
			return false;
		}
		return true;
	}
}

// Returns is_valid and invalid_reason
fn validate_bus_name(name: &Option<String>) -> (bool, String) {
	if ! name.is_some() {
		return (false, String::from("Empty sandbox name"));
	};
	let sandbox_name = name.as_ref().unwrap();
	if sandbox_name.len() > 200 {
		return (false, String::from("Name too long (> 200)"))
	}
	if sandbox_name.contains("-") {
		return (false, String::from("D-Bus names must not contain hyphens"));
	};
	if sandbox_name.contains(".") {
		return (false, String::from("Nested sandbox namespace is undefined behaviour"))
	};


	return (true, String::from(""));
}
