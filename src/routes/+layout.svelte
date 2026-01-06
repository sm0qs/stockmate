<script lang="ts">
	import { getSystemAccentColor } from "$lib/utils/systemColor";
	import "../app.css";
	import "@fontsource-variable/space-grotesk";
	import { onMount } from "svelte";

	onMount(async () => {
		getSystemAccentColor();
	});
	let { children } = $props();
</script>

<main class="p-5">{@render children()}</main>
