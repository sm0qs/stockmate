import fs from "fs";
import { fileURLToPath } from "node:url";
import os from "os";
import path from "path";

export function reset() {
	const PLATFORM = process.platform;
	const HOME = os.homedir();

	try {
		console.log("[reset] Detected platform:", PLATFORM);

		let folders;

		switch (PLATFORM) {
			case "win32":
				folders = [
					path.join(HOME, "AppData", "Roaming", "pl.foxlab.stockmate"),
					path.join(HOME, "AppData", "Local", "pl.foxlab.stockmate", "logs"),
				];
				break;
			case "linux":
				folders = [
					path.join(HOME, ".config", "pl.foxlab.stockmate"),
					path.join(HOME, ".local", "share", "pl.foxlab.stockmate"),
					path.join(HOME, ".local", "share", "stockmate"),
				];
				break;
			default:
				folders = undefined;
				break;
		}

		if (!folders) {
			console.log(`[reset] No specific folders to clean for this platform (${PLATFORM}).`);
			return;
		}

		console.log("[reset] Starting reset...");

		folders.forEach((folder) => {
			fs.rmSync(folder, { recursive: true, force: true });
			console.log(`[reset] removed: ${folder}`);
		});

		console.log("\n[reset] Reset finished successfully.");
	} catch (e) {
		console.error("[reset] Failed to execute reset script:", e.message);
		throw e;
	}
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
	try {
		reset();
	} catch (e) {
		process.exitCode = 1;
	}
}
