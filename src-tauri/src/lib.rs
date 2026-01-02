use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};

pub fn run() {
	tauri::Builder::default()
		// Allow only one instance of the application
		.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
			let _ = app
				.get_webview_window("main")
				.expect("no main window")
				.set_focus();
		}))
		// Logging plugin
		.plugin({
			let mut builder = tauri_plugin_log::Builder::new()
				.clear_targets()
				.timezone_strategy(TimezoneStrategy::UseLocal);

			// Logging in development
			if cfg!(debug_assertions) {
				builder = builder
					.target(Target::new(TargetKind::Stdout))
					.level(log::LevelFilter::Info)
			}
			// Logging in release
			else {
				builder = builder
					.target(Target::new(TargetKind::LogDir {
						file_name: Some("logs".to_string()),
					}))
					.level(log::LevelFilter::Warn)
			}

			builder.build()
		})
		// Prevent webview shortcuts like CTRL+P or CTRL+J in release builds
		.plugin(tauri_plugin_prevent_default::debug())
		// Rust commands to be invoked from the frontend
		.invoke_handler(tauri::generate_handler![])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
