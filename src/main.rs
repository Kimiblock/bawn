use std::process::ExitCode;
mod types;
mod validate_busname;
mod generator;
mod start;

fn main() -> ExitCode {
	let args = std::env::args();
	let args_len = args.len();
	if args_len <= 1 {
		help();
		return std::process::ExitCode::from(1);
	}
	let options = cmdline_dispatcher(args);

	let mut config = types::PortableConfig::new(
		&options.sandbox_name.unwrap(),
	);
	if options.game_mode {
		config.system.deviceAllow = vec!["dgpu".to_string()]
	}

	match options.action {

		types::Action::Start => {
			let result = start::start_portable(&config);
			match result {
				Ok(_string) => {},
				Err(e) => {
					println!(
						"{}",
						e.to_string(),
					);
					return ExitCode::FAILURE
				}
			};
		}

		types::Action::Inspect => {
			config.print();
		}
	};

	ExitCode::SUCCESS
}

fn cmdline_dispatcher(args: std::env::Args) -> types::CmdOptions {
	println!("Bawn started with {} arguments", args.len());
	let mut ret = types::CmdOptions {
		sandbox_name:	None,
		exec_name:	None,
		action:		types::Action::Start,
		game_mode:	false,
	};

	for (idx, argument) in args.enumerate() {
		match idx {
			0 => {ret.exec_name = Some(argument);}
			1 => {
				ret.sandbox_name = Some(argument);
			}
			_ => {
				match argument.as_str() {
					"--inspect" => {
						ret.action = types::Action::Inspect;
					}
					"-g" | "--game-mode" | "--discrete-gpu" => {
						ret.game_mode = true;
					}
					_ => {
						println!(
							"Unrecognised option {}",
							argument,
						);
					}
				}
			}
		};
	};
	let valid = ret.is_valid();
	if ! valid {
		panic!("Could not start sandbox: Invalid sandbox name")
	}
	return ret
}

fn help() {
	println!("This is bawn, a transient profile generator for the Portable sandbox");
	println!("Usage:");
	println!("	bawn <sandbox name> [options]");
	println!("	Note that <option> means required, [option] means optional");
	println!("	Available options:");
	println!("		--inspect: print out generated sandbox configuration");
	println!("		--discrete-gpu / -g: expose all GPUs to the sandbox");
	println!("	All arguments must be valid UTF-8 characters, additional restrictions");
	println!("		apply for sandbox name");
}
