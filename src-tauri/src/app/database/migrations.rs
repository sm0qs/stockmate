use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
	vec![Migration {
		version: 1,
		description: "initial migration",
		sql: include_str!("../../../migrations/1_init.sql"),
		kind: MigrationKind::Up,
	}]
}
