import { rmSync } from "node:fs";
import { fileURLToPath } from "node:url";

export function clean() {
	const FOLDERS = [
		".svelte-kit",
		"build",
		"node_modules",
		"src-tauri/gen",
		"src-tauri/icons",
		"src-tauri/target",
	];

	try {
		console.log("[clean] Starting cleanup...");

		FOLDERS.forEach((folder) => {
			rmSync(folder, { recursive: true, force: true });
			console.log("[clean] removed: " + folder);
		});

		console.log("[clean] Cleanup finished successfully.");
	} catch (e) {
		console.error("[clean] Failed to execute clear script:", e.message);
		throw e;
	}
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
	try {
		clean();
	} catch (e) {
		process.exitCode = 1;
	}
}
