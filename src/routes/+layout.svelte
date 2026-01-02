<script lang="ts">
	import "../app.css";
	import "@fontsource-variable/space-grotesk";

	let { children } = $props();
</script>

<main class="p-5">{@render children()}</main>
