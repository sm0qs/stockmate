<script lang="ts">
	import Navbar from "$lib/components/Navbar.svelte";
	import { getSystemAccentColor } from "$lib/utils/system-color";
	import "./layout.css";
	import { onMount } from "svelte";

	onMount(async () => {
		await getSystemAccentColor();
	});
	let { children } = $props();
</script>

<main class="flex min-h-screen">
	<aside>
		<Navbar />
	</aside>
	<section class="p-5">
		{@render children()}
	</section>
</main>
