<script lang="ts">
	import {
		choosePath,
		cleanPath,
		deletePath,
		fetchPaths,
		openPath,
		savePath,
		savedPaths,
	} from "$lib/utils/path-handler";
	import { FolderOpen, Plus, Trash } from "@lucide/svelte";
	import { onMount } from "svelte";

	let path_input: string = "";

	onMount(fetchPaths);
</script>

<div class="rounded-xl border border-accent p-5">
	<div class="mb-5">
		<h2 class="text-xl font-semibold">Photo directories:</h2>
		<p>Manage directories where the application will look for photos.</p>
	</div>
	<div class="flex items-center gap-2">
		<input
			type="text"
			placeholder="Enter a full directory path..."
			class="rounded border-2 border-accent p-2 outline-none"
			bind:value={path_input}
			on:keydown={(e) => e.key === "Enter" && savePath(path_input)}
		/>

		<button
			type="button"
			class="flex cursor-pointer items-center gap-2 rounded border-2 border-accent bg-accent px-4 py-2 text-on-accent transition hover:border-accent-dark hover:bg-accent-dark hover:text-light"
			on:click={() => savePath(path_input)}
		>
			<Plus />
			Add
		</button>

		<button
			type="button"
			class="flex cursor-pointer items-center gap-2 rounded border-2 border-accent px-4 py-2 transition hover:bg-accent hover:text-on-accent"
			on:click={choosePath}
		>
			<FolderOpen />
			Choose
		</button>
	</div>
	<div class="mt-5 space-y-2">
		{#if $savedPaths.length > 0}
			{#each $savedPaths as p}
				<div
					class="flex items-center justify-between rounded border-2 border-accent px-4 py-2"
				>
					<button
						type="button"
						class="text-left break-all select-all hover:underline"
						on:click={() => openPath(p)}
					>
						{cleanPath(p)}
					</button>

					<button
						type="button"
						class="flex h-8 w-8 cursor-pointer items-center justify-center rounded-lg bg-red-500 hover:bg-red-800"
						on:click={() => deletePath(p)}
					>
						<Trash size={20} />
					</button>
				</div>
			{/each}
		{:else}
			<div class="rounded border-2 border-dashed border-gray-500 p-8 text-gray-500">
				<p class="font-semibold">No photo directories have been added yet.</p>
				<p>Use the input field or the "Choose" button to add a directory.</p>
			</div>
		{/if}
	</div>
</div>
