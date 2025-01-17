use std::path::PathBuf;

use crate::pid::PidConstants;

#[cfg(windows)]
fn get_dll_directory() -> Option<PathBuf> {
	(*DLL_PATH).clone()
}

pub fn get_settings_data() -> PidConstants {
	#[cfg(windows)]
	let dll_directory = match Self::get_dll_directory() {
		Some(dir) => dir,
		None => {
			error!("get_dll_directory() に 失敗しました。");
			return Default::default();
		},
	};
	#[cfg(not(windows))]
	let dll_directory = PathBuf::new();

	let config_path = dll_directory.join("pid-ato.toml");
	let config_data = match std::fs::read_to_string(&config_path) {
		Ok(data) => data,
		Err(_) => {
			return Default::default();
		},
	};
	let settings: PidConstants = match toml::from_str(&config_data) {
		Ok(config) => config,
		Err(_) => {
			return Default::default();
		},
	};
	settings
}