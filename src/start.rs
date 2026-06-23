use crate::types;
//use std::result::Result;

type Result<T> = std::result::Result<T, StartError>;

//#[derive(Debug, Clone)]
pub struct StartError (pub String);

impl std::fmt::Display for StartError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
		-> std::result::Result<(), std::fmt::Error> {
			write!(f, "Could not start Portable: {self}")
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

	Ok(Some(String::new()))
}
