import { invoke } from "@tauri-apps/api/core";
import { writable, derived, get } from "svelte/store";

export const savedPaths = writable<string[]>([]);
export const hasSavedPaths = derived(savedPaths, (v) => v.length > 0);

export async function fetchPaths() {
	const paths = await invoke<string[] | null>("fetch_paths");
	savedPaths.set(paths ?? []);
}

export async function savePath(path: string) {
	if (path.trim() === "" || get(savedPaths).length > 0) {
		return;
	}
	await invoke("save_path", { pathStr: path });
	await fetchPaths();
}

export async function deletePath(path: string) {
	await invoke("delete_path", { pathStr: path });
	await fetchPaths();
}

export async function openPath(path: string) {
	await invoke("open_path", { pathStr: path });
}

export async function choosePath() {
	if (get(savedPaths).length > 0) {
		return;
	}
	await invoke("choose_path", { edit: false });
	await fetchPaths();
}

export function cleanPath(path: string): string {
	const VERBATIM_PREFIX = "\\\\?\\";
	const UNC_PREFIX = "\\\\?\\UNC\\";
	let s = path.trim();

	if (s.startsWith(UNC_PREFIX)) {
		s = "\\\\" + s.substring(UNC_PREFIX.length);
	} else if (s.startsWith(VERBATIM_PREFIX)) {
		s = s.substring(VERBATIM_PREFIX.length);
	}
	return s;
}

export async function editPath(path: string) {
	await invoke("choose_path", { edit: true, oldPathStr: path });
	await fetchPaths();
}
