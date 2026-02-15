<script lang="ts">
	import Navbar from "$lib/components/Navbar.svelte";
	import Titlebar from "$lib/components/Titlebar.svelte";
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

<div class="flex h-screen flex-col">
	<header>
		<Titlebar />
	</header>
	<main class="flex flex-1 overflow-hidden">
		<aside>
			<Navbar />
		</aside>
		<section class="flex-1 overflow-auto p-5">
			{@render children()}
		</section>
	</main>
</div>
