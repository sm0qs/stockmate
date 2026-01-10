mod app;

use app::colors::get_system_accent_color;
use app::database::get_migrations;
use app::paths::{choose_path, delete_path, fetch_paths, open_path, save_path};
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
					.level(log::LevelFilter::Debug)
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
		// Opener plugin to open links in default browser
		.plugin(tauri_plugin_opener::init())
		// SQL plugin for database interactions
		.plugin(
			tauri_plugin_sql::Builder::default()
				.add_migrations("sqlite:data.db", get_migrations())
				.build(),
		)
		// Dialog plugin for folder selection
		.plugin(tauri_plugin_dialog::init())
		// Rust commands to be invoked from the frontend
		.invoke_handler(tauri::generate_handler![
			get_system_accent_color,
			save_path,
			choose_path,
			fetch_paths,
			delete_path,
			open_path
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
