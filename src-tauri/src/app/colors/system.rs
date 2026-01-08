#[cfg(target_os = "windows")]
#[tauri::command]
pub fn get_system_accent_color() -> Option<String> {
	use windows::UI::ViewManagement::{UIColorType, UISettings};
	log::debug!("Fetching system accent color on Windows");

	match UISettings::new() {
		Ok(settings) => match settings.GetColorValue(UIColorType::Accent) {
			Ok(color) => {
				let hex_color = format!("#{:02X}{:02X}{:02X}", color.R, color.G, color.B);
				log::debug!("System accent color fetched: {}", hex_color);
				Some(hex_color)
			}
			Err(error) => {
				log::debug!("Failed to get accent color value: {:?}", error);
				None
			}
		},
		Err(error) => {
			log::debug!("Failed to create UISettings: {:?}", error);
			None
		}
	}
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn get_system_accent_color() -> Option<String> {
	use super::linux_de::get_kde_accent_color;
	use std::env;
	log::debug!("Fetching system accent color on Linux");

	match env::var("XDG_CURRENT_DESKTOP") {
		Ok(desktop_environment) => match desktop_environment.as_str() {
			"KDE" => get_kde_accent_color(),
			_ => {
				log::debug!(
					"Fetching system accent color is not supported for {}",
					desktop_environment
				);
				None
			}
		},
		Err(error) => {
			log::debug!("Failed to read XDG_CURRENT_DESKTOP: {}", error);
			None
		}
	}
}

// TODO: Implement for other OSes
#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
#[tauri::command]
pub fn get_system_accent_color() -> Option<String> {
	use std::env::consts::OS;
	let platform = OS;
	log::debug!(
		"Fetching system accent color is not supported on {}",
		platform
	);
	None
}
