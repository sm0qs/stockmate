import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";

export async function getSystemAccentColor() {
	try {
		const accent = await invoke<string | null>("get_system_accent_color");
		if (accent !== null) {
			document.documentElement.style.setProperty("--color-accent", accent);
		}
	} catch (e) {
		error(`invoke get_system_accent_color failed: ${e}`);
	}
}
