pub struct CmdOptions {
	pub sandbox_name:		Option<String>,
	pub exec_name:			Option<String>,
	pub action:			Action,
}

pub enum Action {
	Start,
}

