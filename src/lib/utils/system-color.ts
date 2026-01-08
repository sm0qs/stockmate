import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";

function hexToRgb(hex: string) {
	const clean = hex.replace("#", "");
	return {
		r: parseInt(clean.slice(0, 2), 16),
		g: parseInt(clean.slice(2, 4), 16),
		b: parseInt(clean.slice(4, 6), 16),
	};
}

function channelToLinear(c: number) {
	const s = c / 255;
	if (s <= 0.03928) {
		return s / 12.92;
	} else {
		return Math.pow((s + 0.055) / 1.055, 2.4);
	}
}

function relativeLuminance(rgb: { r: number; g: number; b: number }) {
	const R = channelToLinear(rgb.r);
	const G = channelToLinear(rgb.g);
	const B = channelToLinear(rgb.b);
	return 0.2126 * R + 0.7152 * G + 0.0722 * B;
}

function getReadableTextColor(accentHex: string) {
	const rgb = hexToRgb(accentHex);
	const L = relativeLuminance(rgb);

	if (L > 0.179) {
		return "#0e141c";
	} else {
		return "#ffffff";
	}
}

export async function getSystemAccentColor() {
	try {
		const accent = await invoke<string | null>("get_system_accent_color");
		if (!accent) return;

		const root = document.documentElement.style;
		const textColor = getReadableTextColor(accent);

		root.setProperty("--color-accent", accent);
		root.setProperty("--color-on-accent", textColor);
	} catch (e) {
		error(`invoke get_system_accent_color failed: ${e}`);
	}
}
