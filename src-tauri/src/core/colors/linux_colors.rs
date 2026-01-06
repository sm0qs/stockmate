#[cfg(target_os = "linux")]
pub fn get_kde_accent_color() -> Option<String> {
	use std::env;
	use std::fs::read_to_string;
	use std::path::PathBuf;

	log::debug!("Fetching KDE accent color");

	let home = env::var("HOME").ok()?;
	let config_path = PathBuf::from(home).join(".config/kdeglobals");

	if !config_path.exists() {
		log::debug!("kdeglobals file does not exist");
		return None;
	}

	let content = match read_to_string(&config_path) {
		Ok(c) => c,
		Err(err) => {
			log::debug!("Failed to read kdeglobals: {}", err);
			return None;
		}
	};

	for line in content.lines() {
		let line = line.trim();

		if let Some(value) = line.strip_prefix("AccentColor=") {
			let mut parts = value.split(',');
			let r: u8 = match parts.next().and_then(|v| v.parse().ok()) {
				Some(v) => v,
				None => {
					log::debug!("Failed to parse RED component");
					return None;
				}
			};

			let g: u8 = match parts.next().and_then(|v| v.parse().ok()) {
				Some(v) => v,
				None => {
					log::debug!("Failed to parse GREEN component");
					return None;
				}
			};

			let b: u8 = match parts.next().and_then(|v| v.parse().ok()) {
				Some(v) => v,
				None => {
					log::debug!("Failed to parse BLUE component");
					return None;
				}
			};

			let hex_color = format!("#{:02X}{:02X}{:02X}", r, g, b);

			log::debug!("System accent color fetched: {}", hex_color);

			return Some(hex_color);
		}
	}

	log::debug!("AccentColor entry not found in kdeglobals");
	None
}
