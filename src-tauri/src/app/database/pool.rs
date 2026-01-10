use sqlx::SqlitePool;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_sql::{DbInstances, DbPool};

const DB_URL: &str = "sqlite:data.db";

pub async fn get_db_pool(app: AppHandle) -> Result<SqlitePool, String> {
	let db_instances: State<'_, DbInstances> = app.state::<DbInstances>();
	let map = db_instances.0.read().await;

	match map.get(DB_URL) {
		Some(pool) => match pool {
			DbPool::Sqlite(pool) => Ok(pool.clone()),
		},
		None => Err("Database pool not found".to_string()),
	}
}
