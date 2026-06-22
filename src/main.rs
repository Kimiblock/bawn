use std::process::ExitCode;

fn main() -> ExitCode {
	let args = std::env::args_os();
	let args_len = args.len();
	let trailing_s: char = 's';
	if args_len <= 1 {
		help();
		return std::process::ExitCode::from(1);
	}
	println!("Bawn started with {} argument{}", args_len, trailing_s);

	ExitCode::SUCCESS
}

fn help() {
	println!("This is bawn, a transient profile generator for the Portable sandbox");
	println!("Usage:");
	println!("	bawn <sandbox name>");
	println!("	Note that <option> means required, [option] means optional");
}
