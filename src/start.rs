use crate::types;
use rand;
use rand::prelude::*;
use command_fds::{CommandFdExt, FdMapping};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StartError {
	#[error("I/O Error: {0}:?")]
	IO(std::io::Error),
	#[error("Invalid configuration: {0}")]
	Config(String),
	#[error("Invalid environment: {0}")]
	Environment(String),
	#[error("Base directories error: {0}:?")]
	XDG(xdg::BaseDirectoriesError),
	#[error("Collision while mapping file descriptors: {0}:?")]
	FDMapping(command_fds::FdMappingCollision),
	#[error("Error while starting Portable: {0}:?")]
	StartPortable(std::io::Error),
}

pub fn start_portable(config: &types::PortableConfig) -> Result<(), StartError> {
	let result = config.to_string();
	let mut content = String::new();
	match result {
		Ok(con) => content.push_str(&con.to_string()),
		Err(e) => {
			return Err(StartError::Config(e.to_string()))
		},
	};


	let xdg_dir = xdg::BaseDirectories::with_prefix("bawn");

	match xdg_dir.has_runtime_directory() {
		true => {}
		false => {
			return Err(
				StartError::Environment(
					"Could not locate XDG Runtime Directory"
						.to_string(),
				)
			);
		}
	}

	let runtime_file = xdg_dir.get_runtime_directory();
	match runtime_file {
		Ok(_rt) => {}
		Err(e) => {
			return Err(
				StartError::XDG(e),
			);
		}
	}

	let mut config_path: std::path::PathBuf = [
		&runtime_file.unwrap(),
	]
		.iter()
		.collect();
	config_path.push("bawn");

	let exists = std::fs::exists(&config_path);
	match exists {
		Ok(true) => {}
		Ok(false) => {
			let result = std::fs::create_dir_all(&config_path);
			match result {
				Ok(()) => {}
				Err(e) => {
					return Err(
						StartError::IO(e)
					)
				}
			}
			println!("Created Runtime Directory for bawn");
		}
		Err(e) => {
			return Err(
				StartError::IO(e),
			)
		}
	}

	println!("Using Runtime Directory: {}", config_path.to_string_lossy());



	let mut retry_counter: u8 = 0;
	let mut rng = rand::rng();
	let rand_file: Result<std::path::PathBuf, StartError> = loop {
		if retry_counter > 5 {
			break Err(
				StartError::Environment(
					"Maximum retry for config path exceeded"
						.to_string(),
				),
			)
		}
		retry_counter+=1;
		let random = &rng.random_range(0..2147483647);
		let mut file_pth: std::path::PathBuf = [&config_path].iter().collect();
		file_pth.push(&config.metadata.appID);
		file_pth.push(random.to_string());
		let exists = std::fs::exists(&file_pth);
		match exists {
			Ok(true) => {continue}
			Ok(false) => {
				let parent = file_pth.parent();
				let parent_path = match parent {
					Some(pth) => {pth}
					None => {
						break Err(
							StartError::Environment(
								"Could not obtain parent path"
									.to_string(),
							)
						)
					}
				};
				let result = std::fs::create_dir_all(&parent_path);
				match result {
					Ok(()) => {}
					Err(e) => {
						break Err(
							StartError::IO(e),
						)
					}
				}
				break Ok(file_pth)
			}
			Err(e) => {
				break Err(StartError::IO(e))
			}
		}
	};

	let rand_file_path = match rand_file {
		Ok(pth) => pth,
		Err(e) => return Err(e),
	};

	let result = std::fs::write(&rand_file_path, &content);
	match result {
		Ok(()) => {}
		Err(e) => {
			return Err(StartError::IO(e))
		}
	}

	let file = std::fs::File::open(&rand_file_path);
	let file = match file {
		Ok(fd) => fd,
		Err(e) => return Err(StartError::IO(e))
	};

	let result = std::fs::remove_file(&rand_file_path);
	match result {
		Ok(result) => result,
		Err(e) => println!("Could not remove temporary file: {}", e)
	};


	let mut command = std::process::Command::new("/usr/bin/portable");
	command.env("PORTABLE_CONF", "/proc/self/fd/25");
	command.arg("--actions");
	command.arg("debug-shell");
	let map_result = command.fd_mappings(
		vec![
			FdMapping{
				parent_fd: file.into(),
				child_fd: 25,
			},
		]
	);
	match map_result {
		Ok(_cmd) => {}
		Err(e) => return Err(StartError::FDMapping(e))
	}

	let spawn = command.spawn();

	let mut child = match spawn {
		Ok(result) => result,
		Err(e) => return Err(
			StartError::StartPortable(e)
		)
	};


	let result = match child.wait() {
		Ok(result) => result,
		Err(e) => return Err(
			StartError::StartPortable(e)
		)
	};

	println!("Portable exited with code {}", result);

	Ok(())
}
