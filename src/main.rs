use std::process::ExitCode;
mod types;

fn main() -> ExitCode {
	let args = std::env::args();
	let args_len = args.len();
	if args_len <= 1 {
		help();
		return std::process::ExitCode::from(1);
	}
	let options = cmdline_dispatcher(args);

	ExitCode::SUCCESS
}

fn cmdline_dispatcher(args: std::env::Args) -> types::CmdOptions {
	println!("Bawn started with {} arguments", args.len());
	let mut ret = types::CmdOptions {
		sandbox_name:	None,
		exec_name:	None,
	};

	for (idx, argument) in args.enumerate() {
		match idx {
			0 => {ret.exec_name = Some(argument);}
			1 => {
				if ret.sandbox_name.is_some() {
					println!("Repeated sandbox name {}", argument)
				} else {
					ret.sandbox_name = Some(argument);
				}
			}
			_ => {todo!();}
		};
	};
	return ret
}

fn help() {
	println!("This is bawn, a transient profile generator for the Portable sandbox");
	println!("Usage:");
	println!("	bawn <sandbox name> [options]");
	println!("	Note that <option> means required, [option] means optional");
}
