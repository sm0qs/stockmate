use crate::app::database::get_db_pool;
use sqlx::{query, query_as};
use std::fs::canonicalize;
use std::path::Path;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub async fn choose_path(app: AppHandle, edit: bool, _old_path_str: Option<String>) {
	let path = app.dialog().file().blocking_pick_folder();
	match path {
		Some(p) => match edit {
			true => match _old_path_str {
				Some(old_path_str) => {
					edit_path(app, old_path_str, p.to_string()).await.ok();
				}
				None => {
					log::warn!("Old path string not provided for edit operation");
				}
			},
			_ => {
				save_path(app, p.to_string()).await.ok();
			}
		},
		None => log::debug!("No path chosen"),
	}
}

#[tauri::command]
pub async fn save_path(app: AppHandle, path_str: String) -> Result<(), String> {
	log::debug!("Saving path: {}", path_str);

	let canonical_path = match canonicalize(&path_str) {
		Ok(p) => p,
		Err(e) => {
			let msg = format!("Invalid path: {}. Error: {}", path_str, e);
			log::warn!("{}", msg);
			return Err(msg);
		}
	};

	if !canonical_path.is_dir() {
		let msg = format!("The provided path is not a directory: {:?}", canonical_path);
		log::warn!("{}", msg);
		return Err(msg);
	}

	let existing = fetch_paths(app.clone()).await.unwrap_or_default();

	if !existing.is_empty() {
		let msg = "A directory is already saved — for now only one is allowed.".to_string();
		log::warn!("{}", msg);
		return Err(msg);
	}

	let pool = get_db_pool(app).await?;

	for existing_path in existing {
		let existing_path_ref = Path::new(&existing_path);

		if canonical_path == existing_path_ref {
			let msg = format!("The provided path is already added: {}", existing_path);
			log::warn!("{}", msg);
			return Err(msg);
		}

		if canonical_path.starts_with(existing_path_ref) {
			let msg = format!(
				"Cannot add subdirectory because parent is already added: {}",
				existing_path
			);
			log::warn!("{}", msg);
			return Err(msg);
		}

		if existing_path_ref.starts_with(&canonical_path) {
			let msg = format!(
				"Cannot add parent directory because a subdirectory is already added: {}",
				existing_path
			);
			log::warn!("{}", msg);
			return Err(msg);
		}
	}

	let res = query("INSERT INTO photo_directories (path) VALUES (?)")
		.bind(canonical_path.to_string_lossy().to_string())
		.execute(&pool)
		.await;
	match res {
		Ok(_) => {}
		Err(e) => {
			let msg = format!("Failed to save path: {}", e);
			log::warn!("{}", msg);
			return Err(msg);
		}
	}
	Ok(())
}

#[tauri::command]
pub async fn fetch_paths(app: AppHandle) -> Result<Vec<String>, String> {
	log::debug!("fetch_paths called");

	let pool = get_db_pool(app).await?;

	let rows = match query_as::<_, (String,)>("SELECT path FROM photo_directories ORDER BY id")
		.fetch_all(&pool)
		.await
	{
		Ok(r) => r,
		Err(e) => {
			let msg = format!("Failed to fetch photo directories: {}", e);
			log::warn!("{}", msg);
			return Err(msg);
		}
	};

	log::debug!("fetch_paths rows = {:?}", rows);

	Ok(rows.into_iter().map(|r| r.0).collect())
}

#[tauri::command]
pub async fn delete_path(app: AppHandle, path_str: String) -> Result<(), String> {
	log::debug!("Deleting path: {}", path_str);

	let pool = get_db_pool(app).await?;

	let res = query("DELETE FROM photo_directories WHERE path = (?)")
		.bind(path_str)
		.execute(&pool)
		.await;

	match res {
		Ok(_) => Ok(()),
		Err(e) => {
			let msg = format!("Failed to delete path: {}", e);
			log::warn!("{}", msg);
			Err(msg)
		}
	}
}

#[tauri::command]
pub fn open_path(app: AppHandle, path_str: String) {
	let _ = app.opener().open_path(path_str, None::<&str>);
}

async fn edit_path(
	_app: AppHandle,
	old_path_str: String,
	new_path_str: String,
) -> Result<(), String> {
	log::debug!("Editing path from: {} to: {}", old_path_str, new_path_str);
	let pool = get_db_pool(_app).await.unwrap();

	let res = query("UPDATE photo_directories SET path = (?) WHERE path = (?)")
		.bind(new_path_str)
		.bind(old_path_str)
		.execute(&pool)
		.await;

	match res {
		Ok(_) => Ok(()),
		Err(e) => {
			let msg = format!("Failed to edit path: {}", e);
			log::warn!("{}", msg);
			Err(msg)
		}
	}
}
