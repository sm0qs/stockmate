<script lang="ts">
	import Navbar from "$lib/components/Navbar.svelte";
	import { fetchPaths } from "$lib/utils/path-handler";
	import { getSystemAccentColor } from "$lib/utils/system-color";
	import "./layout.css";
	import { onMount } from "svelte";

	onMount(async () => {
		await getSystemAccentColor();
		fetchPaths();
	});
	let { children } = $props();
</script>

<main class="flex min-h-screen">
	<aside>
		<Navbar />
	</aside>
	<section class="flex-1 p-5">
		{@render children()}
	</section>
</main>
