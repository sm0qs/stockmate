<script lang="ts">
	import { choosePath, savedPaths } from "$lib/utils/path-handler";
	import { Plus, RefreshCw } from "@lucide/svelte";
</script>

{#if $savedPaths.length === 0}
	<h1 class="mb-5 text-5xl font-semibold">Photos</h1>
	<div>
		<button
			on:click={choosePath}
			class="flex flex-col items-center justify-center rounded-2xl border-2 border-dashed border-accent px-20 py-16 text-xl font-semibold text-accent transition hover:bg-accent hover:text-on-accent"
		>
			<Plus size={48} />
			Add photo directories
		</button>
	</div>
{:else}
	<div class="mb-5 flex w-full items-center justify-between">
		<h1 class="text-5xl font-semibold">Photos</h1>

		<button
			on:click={reloadPhotos}
			class="flex items-center gap-2 rounded border-2 border-accent bg-accent px-4 py-2 text-on-accent transition hover:border-accent-dark hover:bg-accent-dark hover:text-light"
		>
			<RefreshCw size={20} />
			Refresh
		</button>
	</div>
{/if}
