import { syncSvelteKit } from "./prepare.js";
import { execSync } from "node:child_process";
import { fileURLToPath } from "node:url";

export function checkRust() {
	try {
		console.log("[check] Starting Rust check");
		execSync("cargo check", { cwd: "src-tauri", stdio: "inherit" });
		console.log("[check] Rust check finished successfully.");
	} catch (e) {
		console.error("[check] Failed to execute Rust check:", e);
		throw e;
	}
}

export function checkSvelte() {
	syncSvelteKit();
	try {
		console.log("[check] Starting Svelte check");
		execSync("svelte-check --tsconfig ./tsconfig.json", { stdio: "inherit" });
		console.log("[check] Svelte check finished successfully.");
	} catch (e) {
		console.error("[check] Failed to execute Svelte check:", e);
		throw e;
	}
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
	try {
		const args = process.argv.slice(2);
		const runRust = args.includes("--rust");
		const runSvelte = args.includes("--svelte");

		if (runSvelte || (!runRust && !runSvelte)) {
			checkSvelte();
		}
		if (runRust || (!runRust && !runSvelte)) {
			checkRust();
		}
	} catch (e) {
		process.exitCode = 1;
	}
}
