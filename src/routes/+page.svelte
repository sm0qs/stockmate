<script lang="ts">
	import { choosePath, fetchPaths, savedPaths } from "$lib/utils/path-handler";
	import { Plus } from "@lucide/svelte";
	import { onMount } from "svelte";

	onMount(fetchPaths);
</script>

<div class="">
	<h1 class="mb-5 text-5xl font-semibold">Photos</h1>

	{#if $savedPaths.length === 0}
		<div class="">
			<button
				on:click={choosePath}
				class="flex flex-col items-center justify-center rounded-2xl border-2 border-dashed border-accent px-20 py-16 text-xl font-semibold text-accent transition hover:bg-accent hover:text-on-accent"
			>
				<Plus size={48} />
				Add photo directories
			</button>
		</div>
	{:else}
		<div>Photo grid will appear here.</div>
	{/if}
</div>
