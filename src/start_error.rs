use std::{
	env::VarError,
	fmt::{Debug, Error, Formatter},
	io,
	num::ParseIntError,
};

pub(crate) struct StartError {
	msg: String,
}

impl Debug for StartError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		f.write_str(self.msg.as_str())
	}
}

impl From<String> for StartError {
	fn from(str: String) -> Self {
		StartError { msg: str }
	}
}

impl From<rercon::Error> for StartError {
	fn from(e: rercon::Error) -> Self {
		StartError {
			msg: format!("RCON: Could not connect: {}", e.to_string()),
		}
	}
}

impl From<serenity::Error> for StartError {
	fn from(e: serenity::Error) -> Self {
		StartError {
			msg: format!("Discord: Could not connect: {}", e.to_string()),
		}
	}
}

impl From<ParseIntError> for StartError {
	fn from(e: ParseIntError) -> Self {
		StartError {
			msg: format!("Discord: Could not parse channel ID: {}", e.to_string()),
		}
	}
}

impl From<io::Error> for StartError {
	fn from(e: io::Error) -> Self {
		StartError {
			msg: format!("Bridge: Could not start thread: {}", e.to_string()),
		}
	}
}

impl From<VarError> for StartError {
	fn from(e: VarError) -> Self {
		StartError {
			msg: format!("Bridge: Could not read env var: {}", e.to_string()),
		}
	}
}
