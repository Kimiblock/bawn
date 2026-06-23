pub struct LogLevelInfo {
	pub PrettyName:	string,
	pub Level:	u8,
}

enum LogLevel {
	debug = LogLevelInfo{
		PrettyName:	"debug",
		Level:		1,
	},
	info = logle,
	warn = 3,
	critical = 4,
}



fn log(level: LogLevel, msg: String) {
	let prefix: string;
	if level == debug {
		prefix = "\033[0m" + "\033[38;2;125;241;118m" + "[Debug]	" + "\033[0m";
	} else if level = warn {

	}

}
