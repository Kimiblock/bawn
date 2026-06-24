use crate::types;
//use std::fs::File;
//use std::result::Result;

type Result<T> = std::result::Result<T, StartError>;

//#[derive(Debug, Clone)]
pub struct StartError (pub String);

impl std::fmt::Display for StartError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
		-> std::result::Result<(), std::fmt::Error> {
			write!(f, "Could not start Portable: {}", self.0)
		}
}

pub fn start_portable(config: &types::PortableConfig) -> Result<Option<String>> {
	let result = config.to_string();
	let mut content = String::new();
	match result {
		Ok(con) => content.push_str(&con.to_string()),
		Err(e) => {
			return Err(StartError(e.to_string()))
		},
	};

	let xdg_dir = xdg::BaseDirectories::with_prefix("bawn");

	match xdg_dir.has_runtime_directory() {
		true => {}
		false => {
			return Err(
				StartError("Could not find runtime directory".to_string())
			);
		}
	}

	let runtime_file = xdg_dir.get_runtime_directory();
	match runtime_file {
		Ok(_rt) => {}
		Err(e) => {
			return Err(StartError(e.to_string()));
		}
	}

	let mut config_path: std::path::PathBuf = [
		&runtime_file.unwrap(),
	]
		.iter()
		.collect();
	config_path.push("gen");

	let exists = std::fs::exists(&config_path);
	match exists {
		Ok(true) => {}
		Ok(false) => {
			let result = std::fs::create_dir_all(&config_path);
			match result {
				Ok(()) => {}
				Err(e) => {
					return Err(StartError(e.to_string()))
				}
			}
			println!("Created Runtime Directory for bawn");
		}
		Err(e) => {
			return Err(StartError(e.to_string()))
		}
	}





	Ok(Some(String::new()))
}
