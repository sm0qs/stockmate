import { execSync } from "node:child_process";
import { fileURLToPath } from "node:url";

export function syncSvelteKit() {
	try {
		console.log("[prepare] Starting SvelteKit sync...");
		execSync("svelte-kit sync", { stdio: "inherit" });
		console.log("[prepare] SvelteKit sync finished successfully.");
	} catch (e) {
		console.error("[prepare] Failed to execute SvelteKit sync:", e);
		throw e;
	}
}

export function generateIcons() {
	try {
		console.log("[prepare] Starting icon generation...");
		execSync("tauri icon ./static/app-icon.svg", { stdio: "inherit" });
		console.log("[prepare] Icon generation finished successfully.");
	} catch (e) {
		console.error("[prepare] Failed to generate icons:", e.message);
		throw e;
	}
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
	try {
		syncSvelteKit();
		generateIcons();
	} catch (e) {
		process.exitCode = 1;
	}
}
